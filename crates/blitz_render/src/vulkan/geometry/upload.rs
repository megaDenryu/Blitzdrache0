//! ステージングバッファ経由でデバイスローカルバッファへアップロードする
//! (判断20: 頂点/インデックス共通の転送基盤)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::{device_buffer, host_buffer};

pub(crate) fn ステージング経由でアップロードする(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    データ: &[u8],
    用途: vk::BufferUsageFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    let (ステージングバッファ, ステージングメモリ) =
        host_buffer::確保して書き込む(device, メモリプロパティ, データ, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let バイト数 = u64::try_from(データ.len()).unwrap_or_else(|_| panic!("転送データ長がu64に収まらない"));
    let 確保結果 = device_buffer::確保する(device, メモリプロパティ, バイト数, 用途 | vk::BufferUsageFlags::TRANSFER_DST);
    let (先バッファ, 先メモリ) = match 確保結果 {
        Ok(結果) => 結果,
        Err(誤り) => {
            // 安全性: ステージングバッファはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(ステージングバッファ, None) };
            device.メモリを解放する(ステージングメモリ);
            return Err(誤り);
        }
    };

    let コピー結果 =
        ステージングバッファから転送先バッファへコピーする(転送環境, ステージングバッファ, 先バッファ, バイト数);

    // 安全性: 転送実行は完了済みで、ステージングバッファは以降使用しない。
    unsafe { device.destroy_buffer(ステージングバッファ, None) };
    device.メモリを解放する(ステージングメモリ);

    if let Err(誤り) = コピー結果 {
        // 安全性: 転送先バッファはこのスコープの唯一の所有者で、以降使用しない。
        unsafe { device.destroy_buffer(先バッファ, None) };
        device.メモリを解放する(先メモリ);
        return Err(誤り);
    }

    Ok((先バッファ, 先メモリ))
}

fn ステージングバッファから転送先バッファへコピーする(
    転送環境: &転送実行環境,
    ステージングバッファ: vk::Buffer,
    先バッファ: vk::Buffer,
    バイト数: u64,
) -> Result<(), レンダラーエラー> {
    let 一時 = 転送環境.転送コマンドを積み始める()?;
    let 領域 = [vk::BufferCopy::default().size(バイト数)];
    // 安全性: command_bufferは積み込み開始済み。両バッファは直前に生成済みで、
    // コピー長は両方の確保サイズ以下(同一データ長で確保しているため一致)。
    unsafe {
        一時
            .論理デバイス()
            .cmd_copy_buffer(一時.積む先のコマンドバッファ(), ステージングバッファ, 先バッファ, &領域);
    }
    一時.送信して完了を待つ()
}
