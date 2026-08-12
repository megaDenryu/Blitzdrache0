//! グラフィックスパイプライン(dynamic rendering + 動的ビューポート/シザー)の生成・破棄。
//! 材質を読む描画族の本体生成は`material_family`、パイプラインレイアウトの生成と破棄は`layout`にあり、
//! この2つはレイアウトの所有を呼び出し元へ委ねる。自分のレイアウトを抱えたまま持ち回る布のような使い方は`パイプライン`が持つ。

mod aerial_composite_pipeline;
mod color_pass_depth;
mod create;
mod depth_prepass_pipeline;
mod graphics_pipeline;
mod layout;
mod material_family;
mod point_light_shadow_pipeline;
mod shadow_pipeline;
mod sky_pipeline;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

pub(crate) use aerial_composite_pipeline::空中遠近合成パイプライン;
pub(crate) use color_pass_depth::色パスの深度状態;
pub(crate) use layout::{生成する as レイアウトを生成する, 破棄する as レイアウトを破棄する};
pub(crate) use material_family::{
    シャドウのpipelineを生成する, シーンのpipelineを生成する, 深度プリパスのpipelineを生成する
};
pub(crate) use point_light_shadow_pipeline::点光源の影のパイプライン;
pub(crate) use shadow_pipeline::シャドウパイプライン;
pub(crate) use sky_pipeline::空パイプライン;

/// このエンジンが描く全パイプラインの標本数。マルチサンプルは未採用であり、値を1箇所に置いて描画先の一意化と揃える。
pub(crate) const 描画の標本数: vk::SampleCountFlags = vk::SampleCountFlags::TYPE_1;

/// 自分のパイプラインレイアウトを抱えたまま持ち回るパイプライン。布の描画が使う。
pub(crate) struct パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl パイプライン {
    /// 布描画用の変種(判断54): 接線を宣言しない3属性の頂点入力で生成する。
    /// `ディスクリプタlayout一覧`のset1とset2は、布が読まない役割を表す空のレイアウトである。
    pub(crate) fn 布用に生成する(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let layout = layout::生成する(device, ディスクリプタlayout一覧, カメラ相対の基準原点::プッシュ定数範囲())?;
        let 結果 = create::生成する(
            確保係,
            カラー形式,
            深度形式,
            描画の標本数,
            layout,
            シェーダー,
            graphics_pipeline::頂点属性選択::布用,
            色パスの深度状態::より近いものを描いて書く,
        );
        match 結果 {
            Ok(handle) => Ok(Self { handle, layout }),
            Err(誤り) => {
                layout::破棄する(device, layout);
                Err(誤り)
            }
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handleはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_pipeline(self.handle, None) };
        layout::破棄する(device, self.layout);
    }
}
