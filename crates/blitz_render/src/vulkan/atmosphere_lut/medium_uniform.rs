//! 大気散乱媒体を運ぶシェーダー定数バッファ。進行中フレームごとに1本ずつホスト可視で確保する。
//!
//! 注意: 1本を共有せずスロットごとに持つのは、大気が変わったフレームで書き換えるとき、別のスロットで
//! まだ実行中の生成パスが同じバッファを読んでいる可能性があるためである。スロットごとに分ければ、
//! そのスロットのフェンス待機の後に書く規律だけでこの競合が消える(フレームシェーダー定数と同じ理由)。

use ash::vk;

use super::medium_bytes;
use crate::atmosphere::大気散乱媒体;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, フレームスロットごとのバッファ};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 媒体シェーダー定数一式 {
    スロットごとのバッファ: フレームスロットごとのバッファ,
}

impl 媒体シェーダー定数一式 {
    pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>) -> Result<Self, レンダラーエラー> {
        let 初期バイト列 = [0u8; medium_bytes::バイト長];
        let スロットごとのバッファ = 確保係.フレームスロットごとのホスト可視バッファを確保して書き込む(
            &初期バイト列,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )?;
        Ok(Self {
            スロットごとのバッファ
        })
    }

    pub(super) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.スロットごとのバッファ.スロットのバッファ(フレーム添字)
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        self.スロットごとのバッファ
            .スロットの中身を書き換える(device, フレーム添字, &medium_bytes::バイト列にする(媒体))
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.スロットごとのバッファ.破棄する(device);
    }
}
