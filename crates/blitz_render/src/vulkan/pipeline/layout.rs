//! パイプラインが宣言するディスクリプタセットの並びとプッシュ定数の範囲を、1つのVkPipelineLayoutとして所有する資源型。
//! 誰がこのレイアウトを持つかは呼び出し元が決め、ここは確保と破棄の対だけを引き受ける。
//!
//! パイプラインの生成から分けるのは、1つのレイアウトを複数のパイプラインが共有するためである。
//! 同じ族のパイプラインがレイアウトを共有できることが、パイプラインを切り替えてもディスクリプタセットの束縛が
//! 生き続ける根拠になる(参照: `vulkan::pipeline_ledger::layouts`)。
//!
//! 注意: `Drop`を持たない。破棄の順番は、このレイアウトで作ったパイプラインの持ち主が決める。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct パイプラインレイアウト(vk::PipelineLayout);

impl パイプラインレイアウト {
    pub(crate) fn 確保する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        プッシュ定数範囲: vk::PushConstantRange,
    ) -> Result<Self, レンダラーエラー> {
        let プッシュ定数範囲一覧 = [プッシュ定数範囲];
        let create_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(ディスクリプタlayout一覧)
            .push_constant_ranges(&プッシュ定数範囲一覧);
        // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
        Ok(Self(unsafe {
            確保係.論理デバイス().create_pipeline_layout(&create_info, None)?
        }))
    }

    /// パイプライン生成・ディスクリプタセットの束縛・プッシュ定数の積み込みへ渡す境界。
    pub(crate) const fn レイアウトのハンドル(&self) -> vk::PipelineLayout {
        self.0
    }

    /// 前提: このレイアウトを参照するパイプラインをすべて破棄した後に呼ぶ。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: レイアウトはSelfが唯一の所有者であり、参照するパイプラインが残っていないことを呼び出し元が保証する。
        unsafe { device.destroy_pipeline_layout(self.0, None) };
    }
}
