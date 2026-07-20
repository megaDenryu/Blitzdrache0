//! 布バッファの生成手順。途中失敗時は確保済みを片付ける(skinningのbuffers/createと同じ台帳方式)。

use ash::vk;

use super::布バッファ;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::{device_buffer, host_buffer};

use super::super::params;

const セル総数: u64 = 32 * 32 * 32;
const セル容量: u64 = 8;

pub(crate) fn 生成する(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    素材: &布素材,
) -> Result<布バッファ, レンダラーエラー> {
    let mut 確保済み: Vec<(vk::Buffer, vk::DeviceMemory)> = Vec::new();
    let 粒子数 = u64::from(素材.粒子数);
    let ストレージ = vk::BufferUsageFlags::STORAGE_BUFFER;

    let 粒子 = 積む(&mut 確保済み, device, upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &素材.粒子バイト列, ストレージ))?;
    let 前位置 = 積む(&mut 確保済み, device, device_buffer::確保する(device, メモリプロパティ, 粒子数 * 16, ストレージ))?;
    let 隣接 = 積む(&mut 確保済み, device, upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &素材.隣接バイト列, ストレージ))?;
    let セルカウント = 積む(&mut 確保済み, device, device_buffer::確保する(device, メモリプロパティ, セル総数 * 4, ストレージ))?;
    let セル格納 = 積む(&mut 確保済み, device, device_buffer::確保する(device, メモリプロパティ, セル総数 * セル容量 * 4, ストレージ))?;
    let 布頂点 = 積む(
        &mut 確保済み,
        device,
        device_buffer::確保する(device, メモリプロパティ, 粒子数 * 48, ストレージ | vk::BufferUsageFlags::VERTEX_BUFFER),
    )?;

    let mut インデックスバイト列 = Vec::with_capacity(素材.インデックス一覧.len() * 4);
    for 添字 in &素材.インデックス一覧 {
        インデックスバイト列.extend_from_slice(&添字.to_le_bytes());
    }
    let インデックス = 積む(&mut 確保済み, device, upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &インデックスバイト列, vk::BufferUsageFlags::INDEX_BUFFER))?;

    let mut アタッチバイト列 = Vec::with_capacity(素材.アタッチ対応一覧.len() * 8);
    for 対応 in &素材.アタッチ対応一覧 {
        アタッチバイト列.extend_from_slice(&対応[0].to_le_bytes());
        アタッチバイト列.extend_from_slice(&対応[1].to_le_bytes());
    }
    let アタッチ = 積む(&mut 確保済み, device, upload::ステージング経由でアップロードする(device, メモリプロパティ, 転送環境, &アタッチバイト列, ストレージ))?;

    let 介入初期値 = vec![0u8; usize::try_from(params::介入上限件数).unwrap_or_else(|_| panic!("介入上限件数がusizeに収まらない")) * 32];
    let 定数初期値 = vec![0u8; params::バイト長];
    let mut 介入一覧 = [(vk::Buffer::null(), vk::DeviceMemory::null()); フレームインフライト数];
    let mut 定数一覧 = [(vk::Buffer::null(), vk::DeviceMemory::null()); フレームインフライト数];
    for 添字 in 0..フレームインフライト数 {
        介入一覧[添字] = 積む(&mut 確保済み, device, host_buffer::確保して書き込む(device, メモリプロパティ, &介入初期値, ストレージ))?;
        定数一覧[添字] = 積む(&mut 確保済み, device, host_buffer::確保して書き込む(device, メモリプロパティ, &定数初期値, vk::BufferUsageFlags::UNIFORM_BUFFER))?;
    }

    Ok(布バッファ { 粒子, 前位置, 隣接, セルカウント, セル格納, 布頂点, インデックス, アタッチ, 介入一覧, 定数一覧 })
}

/// 確保結果を台帳へ積む。失敗時は台帳の確保済みバッファを全部片付けてからエラーを返す。
fn 積む(
    確保済み: &mut Vec<(vk::Buffer, vk::DeviceMemory)>,
    device: &ash::Device,
    結果: Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー>,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    match 結果 {
        Ok(組) => {
            確保済み.push(組);
            Ok(組)
        }
        Err(誤り) => {
            // 安全性: 確保済みバッファはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                for &(buffer, memory) in 確保済み.iter() {
                    device.destroy_buffer(buffer, None);
                    device.free_memory(memory, None);
                }
            }
            Err(誤り)
        }
    }
}
