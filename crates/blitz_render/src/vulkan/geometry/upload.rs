//! ステージングバッファ経由でデバイスローカルバッファへアップロードする
//! (判断20: 頂点/インデックス共通の転送基盤)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::{device_buffer, host_buffer};

pub(super) fn ステージング経由でアップロードする(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    データ: &[u8],
    用途: vk::BufferUsageFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    let (ステージングバッファ, ステージングメモリ) =
        host_buffer::確保して書き込む(device, メモリプロパティ, データ, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let バイト数 = u64::try_from(データ.len()).unwrap_or_else(|_| panic!("転送データ長がu64に収まらない"));
    let 確保結果 =
        device_buffer::確保する(device, メモリプロパティ, バイト数, 用途 | vk::BufferUsageFlags::TRANSFER_DST);
    let (先バッファ, 先メモリ) = match 確保結果 {
        Ok(結果) => 結果,
        Err(誤り) => {
            // 安全性: ステージングバッファ・メモリはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_buffer(ステージングバッファ, None);
                device.free_memory(ステージングメモリ, None);
            }
            return Err(誤り);
        }
    };

    let コピー結果 = 転送環境.一括実行する(device, |command_buffer| {
        let 領域 = vk::BufferCopy::default().size(バイト数);
        // 安全性: command_bufferは転送実行環境が記録用に開始済み。両バッファは直前に
        // 生成済みで、コピー長は両方の確保サイズ以下(同一データ長で確保しているため一致)。
        unsafe { device.cmd_copy_buffer(command_buffer, ステージングバッファ, 先バッファ, &[領域]) };
    });

    // 安全性: ステージングバッファ・メモリはこのスコープの唯一の所有者で、
    // 転送完了(一括実行するがfence待ち済み)後は不要。
    unsafe {
        device.destroy_buffer(ステージングバッファ, None);
        device.free_memory(ステージングメモリ, None);
    }

    if let Err(誤り) = コピー結果 {
        // 安全性: 先バッファ・先メモリはこのスコープの唯一の所有者で、以降使用しない。
        unsafe {
            device.destroy_buffer(先バッファ, None);
            device.free_memory(先メモリ, None);
        }
        return Err(誤り);
    }

    Ok((先バッファ, 先メモリ))
}
