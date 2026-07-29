//! 空パイプライン: 全画面三角形で深度がクリア値のままの画素だけへ空を描く、深度書き込みのないグラフィックスパイプライン。
//! シーンのディスクリプタセットレイアウトをそのまま再利用する(シャドウパイプラインと同じ方針)。
//! フレームユニフォームのbinding3だけを読み、テクスチャは1枚も読まない。

mod assemble;
mod create;
mod finish;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

pub(crate) struct 空パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 空パイプライン {
    /// `カラー形式`はシーンと同じ色アタッチメント(ポスト処理があればHDR中間画像)の形式、
    /// `深度形式`はシーンが書いた深度をそのまま読むための形式である。
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, カラー形式, 深度形式, ディスクリプタlayout, シェーダー)
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
