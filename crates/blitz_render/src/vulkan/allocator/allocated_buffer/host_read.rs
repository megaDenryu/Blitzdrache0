//! GPUが書いた内容をホストへ写し取る操作。担当するのは写像・複写・写像の解放という対の手順であり、
//! 写し取ったバイト列や語列が何を表すかは呼び出し元が持つ。
//!
//! 前提: このバッファはホスト可視かつコヒーレントなメモリで確保されている。
//! 注意: 呼び出し元は、このバッファへのGPUの書き込み(コピーコマンド)がフェンス待機で完了済みであることを保証する。
//! ここでは写像の前の同期を行わない。

use ash::vk;

use super::専用メモリ付きバッファ;
use crate::error::レンダラーエラー;

/// 32ビット語1つのバイト長。写し取る語数から触るバイト数を求めるために持つ。
const 語1つのバイト長: usize = 4;

impl 専用メモリ付きバッファ {
    /// 先頭から指定した長さのバイト列を写し取る。長さが確保量を超えていればこの型が止める。
    pub(crate) fn ホスト可視のバイト列を写し取る(
        &self,
        device: &ash::Device,
        バイト数: usize,
    ) -> Result<Vec<u8>, レンダラーエラー> {
        self.写像で触る長さが確保量に収まることを確かめる(バイト数);
        // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済み。WHOLE_SIZEでの写像は確保量全体を対象にする。
        let ポインタ = unsafe { device.map_memory(self.memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
        // 安全性: ポインタは写像が返した確保量全体の先頭であり、読む長さが確保量以下であることを呼び出し元が保証する。HOST_COHERENTのためinvalidateは不要。
        let バイト列 = unsafe { std::slice::from_raw_parts(ポインタ.cast::<u8>(), バイト数) }.to_vec();
        // 安全性: memoryはこの直前に写像済みの同一ハンドルである。
        unsafe { device.unmap_memory(self.memory) };
        Ok(バイト列)
    }

    /// 先頭から指定した数の32ビット語を写し取る。
    /// 語のまま返すのは、同じ列に件数(符号なし整数)と補正段(単精度)が混じり、意味づけを呼び出し元が持つためである。
    /// 語数の4倍が確保量を超えていればこの型が止める。
    pub(crate) fn ホスト可視の32ビット語列を写し取る(
        &self,
        device: &ash::Device,
        語数: usize,
    ) -> Result<Vec<u32>, レンダラーエラー> {
        self.写像で触る長さが確保量に収まることを確かめる(語数 * 語1つのバイト長);
        // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済み。WHOLE_SIZEでの写像は確保量全体を対象にする。
        let ポインタ = unsafe { device.map_memory(self.memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
        // 安全性: ポインタは写像が返した確保量全体の先頭であり、4バイト境界に整列している。読む長さが確保量以下であることを呼び出し元が保証する。
        let 語列 = unsafe { std::slice::from_raw_parts(ポインタ.cast::<u32>(), 語数) }.to_vec();
        // 安全性: memoryはこの直前に写像済みの同一ハンドルである。
        unsafe { device.unmap_memory(self.memory) };
        Ok(語列)
    }
}
