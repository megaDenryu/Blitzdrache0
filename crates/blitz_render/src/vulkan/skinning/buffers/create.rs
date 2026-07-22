//! スキニングバッファの生成手順。途中失敗時は確保済みのバッファを逆順で片付ける。

mod allocation;
mod attribute_bytes;

use ash::vk;

use self::allocation::積む;
use self::attribute_bytes::属性をバイト列にする;
use super::スキニングバッファ;
use crate::error::レンダラーエラー;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan::geometry::{bytes, upload};
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::{device_buffer, host_buffer};

const 行列バイト長: usize = 64;

pub(crate) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    頂点一覧: &[頂点],
    素材: &スキンメッシュ素材,
) -> Result<スキニングバッファ, レンダラーエラー> {
    let mut 確保済み: Vec<(vk::Buffer, vk::DeviceMemory)> = Vec::new();

    let 頂点バイト列 = bytes::頂点をバイト列にする(頂点一覧);
    let レスト頂点 = 積む(
        &mut 確保済み,
        device,
        upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &頂点バイト列,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        ),
    )?;

    let 属性バイト列 = 属性をバイト列にする(素材);
    let 属性 = 積む(
        &mut 確保済み,
        device,
        upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &属性バイト列,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        ),
    )?;

    let 行列初期値 = vec![0u8; 素材.ジョイント数() * 行列バイト長];
    let mut 行列一覧 = [(vk::Buffer::null(), vk::DeviceMemory::null()); フレームインフライト数];
    for 行列 in &mut 行列一覧 {
        *行列 = 積む(
            &mut 確保済み,
            device,
            host_buffer::確保して書き込む(device, メモリプロパティ, &行列初期値, vk::BufferUsageFlags::STORAGE_BUFFER),
        )?;
    }

    // スキン済み頂点: コンピュートが書き、シーン/シャドウが頂点入力として読む(判断44の合流点)。
    let 出力サイズ = u64::try_from(頂点バイト列.len()).unwrap_or_else(|_| panic!("出力バッファ長がu64に収まらない"));
    let 出力 = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(
            device,
            メモリプロパティ,
            出力サイズ,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::VERTEX_BUFFER,
        ),
    )?;

    Ok(スキニングバッファ {
        レスト頂点,
        属性,
        行列一覧,
        出力,
    })
}
