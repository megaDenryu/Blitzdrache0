//! パイプラインレイアウトの生成と破棄。担当するのは、ディスクリプタセットレイアウトの並びとプッシュ定数の範囲から
//! VkPipelineLayoutを作る1工程であり、誰がそれを所有するかは呼び出し元が決める。
//!
//! パイプラインの生成から分けるのは、1つのレイアウトを複数のパイプラインが共有するためである。
//! 同じ族のパイプラインがレイアウトを共有できることが、パイプラインを切り替えてもディスクリプタセットの束縛が
//! 生き続ける根拠になる(参照: `vulkan::pipeline_ledger::layouts`)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 生成する(
    device: &ash::Device,
    ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
    プッシュ定数範囲: vk::PushConstantRange,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let プッシュ定数範囲一覧 = [プッシュ定数範囲];
    let create_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(ディスクリプタlayout一覧)
        .push_constant_ranges(&プッシュ定数範囲一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&create_info, None)? };
    Ok(layout)
}

/// 前提: このレイアウトを参照するパイプラインをすべて破棄した後に呼ぶ。
pub(crate) fn 破棄する(device: &ash::Device, layout: vk::PipelineLayout) {
    // 安全性: layoutは呼び出し元が唯一の所有者であり、参照するパイプラインが残っていないことを保証する。
    unsafe { device.destroy_pipeline_layout(layout, None) };
}
