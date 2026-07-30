//! グラフィックスパイプライン(dynamic rendering + 動的ビューポート/シザー)の生成・破棄。
//! 頂点/インデックスバッファから立方体を描画し、深度テストとプッシュ定数
//! （ビュー射影行列）を持つ。

mod aerial_composite_pipeline;
mod create;
mod graphics_pipeline;
mod shadow_pipeline;
mod sky_pipeline;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

pub(crate) use aerial_composite_pipeline::空中遠近合成パイプライン;
pub(crate) use shadow_pipeline::シャドウパイプライン;
pub(crate) use sky_pipeline::空パイプライン;

/// 立方体描画1本ぶんのパイプラインと、プッシュ定数を送るためのパイプラインレイアウト。
pub(crate) struct パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl パイプライン {
    /// `カラー形式` はスワップチェーンの、`深度形式` は深度バッファの
    /// dynamic renderingの出力形式。`ディスクリプタlayout` はベースカラーテクスチャの
    /// combined image sampler(判断21)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(
            device,
            カラー形式,
            深度形式,
            ディスクリプタlayout,
            シェーダー,
            graphics_pipeline::頂点属性選択::全属性,
        )
    }

    /// 布描画用の変種(判断54): 接線を宣言しない3属性の頂点入力で生成する。
    pub(crate) fn 布用に生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(
            device,
            カラー形式,
            深度形式,
            ディスクリプタlayout,
            シェーダー,
            graphics_pipeline::頂点属性選択::布用,
        )
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
