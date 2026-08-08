//! GPU上のバッファを32ビット語の列としてホストへ読み戻す工程。受け取るのは読み戻し元とバイト数、返すのは語の列である。
//! 呼ばれるのは検収の実行が求めたときだけであり、本番のフレーム経路は1度も通らない。
//!
//! 画像の読み戻しと分けるのは、対象に寸法も画素の並びも無く、成分の型を決めるのが呼び出し元だからである。
//! 語で返すのは、同じバイト列に件数(符号なし整数)と補正段(単精度)が混じるためであり、意味づけは呼び出し元が持つ。

use ash::vk;

use super::buffer::読み戻しバッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

/// 前提: 呼び出し時点でGPUが読み戻し元のバッファを使用していないこと(device_wait_idle後)。
pub(crate) fn 語列を読み戻す(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    元: vk::Buffer,
    バイト数: u64,
) -> Result<Vec<u32>, レンダラーエラー> {
    let 受け皿 = 読み戻しバッファ::生成する(device, メモリプロパティ, バイト数)?;
    let 結果 = 転送して開く(device, 転送環境, &受け皿, 元, バイト数);
    受け皿.破棄する(device);
    結果
}

fn 転送して開く(
    device: &GPUデバイス,
    転送環境: &転送実行環境,
    受け皿: &読み戻しバッファ,
    元: vk::Buffer,
    バイト数: u64,
) -> Result<Vec<u32>, レンダラーエラー> {
    読み戻し元を受け皿へコピーする(転送環境, 元, 受け皿.handle, バイト数)?;
    let 要素数 = usize::try_from(バイト数 / 4).unwrap_or_else(|_| panic!("読み戻す語数がusizeに収まらない: {バイト数}"));
    // 安全性: 受け皿のメモリはHOST_VISIBLE|HOST_COHERENTで確保済みで、写す長さは確保長ぴったりである。
    let ポインタ = unsafe { device.map_memory(受け皿.memory(), 0, バイト数, vk::MemoryMapFlags::empty())? };
    // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、要素数ぶんだけを読む。転送は直前のfence待機で完了済みである。
    let 語列 = unsafe { std::slice::from_raw_parts(ポインタ.cast::<u32>(), 要素数) }.to_vec();
    // 安全性: memoryはこの直前にmap_memory済みの同一ハンドルである。
    unsafe { device.unmap_memory(受け皿.memory()) };
    Ok(語列)
}

fn 読み戻し元を受け皿へコピーする(
    転送環境: &転送実行環境,
    元: vk::Buffer,
    受け皿: vk::Buffer,
    バイト数: u64,
) -> Result<(), レンダラーエラー> {
    let 一時 = 転送環境.転送コマンドを積み始める()?;
    let 領域 = [vk::BufferCopy::default().size(バイト数)];
    // 安全性: command_bufferは積み込み開始済みで、コピー長は両バッファの確保長以下である。
    unsafe { 一時.論理デバイス().cmd_copy_buffer(一時.積む先のコマンドバッファ(), 元, 受け皿, &領域) };
    一時.送信して完了を待つ()
}
