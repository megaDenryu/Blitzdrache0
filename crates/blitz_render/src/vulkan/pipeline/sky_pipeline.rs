//! 空パイプライン: 全画面三角形で深度がクリア値のままの画素だけへ空を描く、深度書き込みのないグラフィックスパイプライン。
//! set0はscene系と同じビューとパスのセット(空パス定数を含む)であり、set1では大気のベイク済み画像を参照する
//! ディスクリプタセットレイアウトを常に足す。

mod assemble;
mod create;
mod finish;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 空パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 空パイプライン {
    /// `カラー形式`はシーンと同じ色アタッチメント(ポスト処理があればHDR中間画像)の形式、
    /// `深度形式`はシーンが書いた深度をそのまま読むための形式である。
    /// `ディスクリプタlayout一覧`はset0から順に並べたレイアウトであり、set0はビューとパスのセット、
    /// set1は大気のベイク済み画像を参照するレイアウトである。
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, カラー形式, 深度形式, ディスクリプタlayout一覧, シェーダー)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
