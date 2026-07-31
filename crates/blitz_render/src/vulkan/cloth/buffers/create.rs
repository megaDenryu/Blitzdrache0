//! 布バッファの生成手順。途中失敗時は確保済みを片付ける(skinningのbuffers/createと同じ台帳方式)。

mod allocation;
mod input_buffers;

use ash::vk;

use self::allocation::積む;
use super::布バッファ;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::sync::進行中フレーム数;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::{device_buffer, host_buffer};

use super::super::params;

const セル総数: u64 = 32 * 32 * 32;
const セル容量: u64 = 8;

pub(crate) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    素材: &布素材,
) -> Result<布バッファ, レンダラーエラー> {
    let mut 確保済み: Vec<(vk::Buffer, vk::DeviceMemory)> = Vec::new();
    let 粒子数 = u64::from(素材.粒子数);
    let ストレージ = vk::BufferUsageFlags::STORAGE_BUFFER;

    let 粒子 = 積む(
        &mut 確保済み,
        device,
        upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &素材.粒子バイト列, ストレージ),
    )?;
    let 前位置 = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(device, メモリプロパティ, 粒子数 * 16, ストレージ),
    )?;
    let 隣接 = 積む(
        &mut 確保済み,
        device,
        upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &素材.隣接バイト列, ストレージ),
    )?;
    let セルカウント = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(device, メモリプロパティ, セル総数 * 4, ストレージ),
    )?;
    let セル格納 = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(device, メモリプロパティ, セル総数 * セル容量 * 4, ストレージ),
    )?;
    let 布頂点 = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(device, メモリプロパティ, 粒子数 * 48, ストレージ | vk::BufferUsageFlags::VERTEX_BUFFER),
    )?;

    let (インデックス, アタッチ) = input_buffers::生成する(&mut 確保済み, device, メモリプロパティ, 転送環境, 素材, ストレージ)?;

    let 介入初期値 =
        vec![0u8; usize::try_from(params::介入上限件数).unwrap_or_else(|_| panic!("介入上限件数がusizeに収まらない")) * 32];
    let 定数初期値 = vec![0u8; params::バイト長];
    let mut 介入一覧 = [(vk::Buffer::null(), vk::DeviceMemory::null()); 進行中フレーム数];
    let mut 定数一覧 = [(vk::Buffer::null(), vk::DeviceMemory::null()); 進行中フレーム数];
    for 添字 in 0..進行中フレーム数 {
        介入一覧[添字] = 積む(
            &mut 確保済み,
            device,
            host_buffer::確保して書き込む(device, メモリプロパティ, &介入初期値, ストレージ),
        )?;
        定数一覧[添字] = 積む(
            &mut 確保済み,
            device,
            host_buffer::確保して書き込む(device, メモリプロパティ, &定数初期値, vk::BufferUsageFlags::UNIFORM_BUFFER),
        )?;
    }

    Ok(布バッファ {
        粒子,
        前位置,
        隣接,
        セルカウント,
        セル格納,
        布頂点,
        インデックス,
        アタッチ,
        介入一覧,
        定数一覧,
    })
}
