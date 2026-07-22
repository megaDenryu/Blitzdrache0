//! ホスト可視バッファのマッピングと、読み戻し画像への変換(BGRA→RGBA並び替え含む)。
//!
//! 注意: 呼び出し元は、このバッファへのGPU書き込み(コピーコマンド)がフェンス待機で
//! 完了済みであることを保証すること。ここではマッピング前の同期は行わない。

use ash::vk;

use super::buffer::読み戻しバッファ;
use crate::error::レンダラーエラー;
use crate::readback_image::読み戻し画像;

pub(crate) fn 読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
    形式: vk::Format,
) -> Result<読み戻し画像, レンダラーエラー> {
    let 必要バイト数: u64 = u64::from(寸法.width) * u64::from(寸法.height) * 4;
    let 要素数 = usize::try_from(必要バイト数).unwrap_or_else(|_| panic!("読み戻しバイト数がusizeに収まらない: {必要バイト数}"));

    // 安全性: バッファのメモリはHOST_VISIBLE|HOST_COHERENTで確保済みで、
    // 必要バイト数はバッファ容量以下であることを呼び出し元が保証する。
    let ポインタ = unsafe { device.map_memory(バッファ.memory(), 0, 必要バイト数, vk::MemoryMapFlags::empty())? };
    // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、要素数ぶんのみ読む。
    // HOST_COHERENTのためinvalidateは不要。
    let 生データ = unsafe { std::slice::from_raw_parts(ポインタ.cast::<u8>(), 要素数) }.to_vec();
    // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
    unsafe { device.unmap_memory(バッファ.memory()) };

    let rgba = if bgra形式か(形式) {
        並び替える(&生データ)
    } else {
        生データ
    };
    Ok(読み戻し画像::生成する(寸法.width, 寸法.height, rgba))
}

fn 並び替える(データ: &[u8]) -> Vec<u8> {
    let mut 結果 = データ.to_vec();
    for 画素 in 結果.chunks_exact_mut(4) {
        画素.swap(0, 2);
    }
    結果
}

fn bgra形式か(形式: vk::Format) -> bool {
    matches!(
        形式,
        vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB | vk::Format::B8G8R8A8_SNORM | vk::Format::B8G8R8A8_UINT | vk::Format::B8G8R8A8_SINT
    )
}
