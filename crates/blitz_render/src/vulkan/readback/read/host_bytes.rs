//! ホスト可視バッファをマップして生のバイト列を写す局面。触れるのは読み戻しバッファのメモリだけであり、
//! 成分の型も並びの意味も知らない。
//!
//! 対象ごとの復号と分けるのは、マップと解放という対の操作をこの1箇所に閉じるためである。復号のたびに
//! マップを書くと、途中で失敗する経路が増えるほど解放の抜けが生まれる。
//!
//! 注意: 呼び出し元は、このバッファへのGPU書き込み(コピーコマンド)がフェンス待機で完了済みであることを
//! 保証すること。ここではマッピング前の同期を行わない。

use ash::vk;

use super::super::buffer::読み戻しバッファ;
use super::super::target::読み戻し対象;
use crate::error::レンダラーエラー;

pub(super) fn 読む(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
    対象: 読み戻し対象,
) -> Result<Vec<u8>, レンダラーエラー> {
    let 必要バイト数 = u64::from(寸法.width) * u64::from(寸法.height) * 対象.画素あたりバイト数();
    let 要素数 = usize::try_from(必要バイト数).unwrap_or_else(|_| panic!("読み戻しバイト数がusizeに収まらない: {必要バイト数}"));

    // 安全性: バッファのメモリはHOST_VISIBLE|HOST_COHERENTで確保済みで、必要バイト数はバッファ容量以下であることを呼び出し元が保証する。
    let ポインタ = unsafe { device.map_memory(バッファ.memory(), 0, 必要バイト数, vk::MemoryMapFlags::empty())? };
    // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、要素数ぶんのみ読む。
    // HOST_COHERENTのためinvalidateは不要。
    let 生データ = unsafe { std::slice::from_raw_parts(ポインタ.cast::<u8>(), 要素数) }.to_vec();
    // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
    unsafe { device.unmap_memory(バッファ.memory()) };
    Ok(生データ)
}
