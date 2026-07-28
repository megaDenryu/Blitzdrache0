//! 毎フレームの可視ID列の書き込み局面。フェンス待機の済んだフレームスロットのバッファへ、そのフレームの列を書く。
//! 触れるのはこのバッファが所有するホスト可視メモリだけであり、確保も解放も行わない。
//! 列は全個体の並べ替えであるため、書き込む長さは確保時の個体数と常に等しい。長さが個体数と一致することは
//! 呼び出し元の描画対象資源が検査済みであり、ここでは確保容量を超えないことだけを最後の砦として確かめる。

use ash::vk;

use super::{可視IDバイト長, 可視ID列バッファ};
use crate::error::レンダラーエラー;
use crate::visible_instance_selection::可視ID列エラー;
use crate::vulkan::sync::フレームスロット添字;

impl 可視ID列バッファ {
    pub(crate) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        可視id列: &[u32],
    ) -> Result<(), レンダラーエラー> {
        if 可視id列.len() > usize::try_from(self.個体数).unwrap_or(usize::MAX) {
            return Err(可視ID列エラー::列の長さ不一致 {
                列の長さ: 可視id列.len(),
                個体数: self.個体数,
            }
            .into());
        }
        if 可視id列.is_empty() {
            return Ok(());
        }
        let memory = self.memory(フレーム添字);
        // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済みであり、書き込む長さは確保時の個体数ぶん以下である。
        let ポインタ = unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
        // 安全性: ポインタはmap_memoryが返した確保容量全体の先頭であり、書き込む範囲は上の検査でその内側に収まる。
        let 書込先 = unsafe { std::slice::from_raw_parts_mut(ポインタ.cast::<u8>(), 可視id列.len() * 可視IDバイト長) };
        for (区画, 可視id) in 書込先.chunks_exact_mut(可視IDバイト長).zip(可視id列) {
            区画.copy_from_slice(&可視id.to_le_bytes());
        }
        // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
        unsafe { device.unmap_memory(memory) };
        Ok(())
    }
}
