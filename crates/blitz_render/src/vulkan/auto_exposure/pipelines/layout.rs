//! 自動露出の2本のコンピュートが押し込む即時定数の大きさと、その大きさを持つパイプラインレイアウトを所有する。
//! 触れる状態はレイアウト1つに閉じており、呼び出し側は押し込む定数の並びを知らずに「集計の」「導出の」と名指すだけで呼べる。

use ash::vk;

use crate::error::レンダラーエラー;

/// 押し込む定数のバイト数。`shaders/auto_exposure_histogram.slang`の`HistogramExtent`(32ビット2つ)と`auto_exposure_resolve.slang`の`ResolveSetting`(単精度6つ)に一致させる。
const 集計の即時定数バイト数: u32 = 8;
const 導出の即時定数バイト数: u32 = 24;

pub(super) fn 集計のレイアウトを作る(
    device: &ash::Device,
    セットレイアウト: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    作る(device, セットレイアウト, 集計の即時定数バイト数)
}

pub(super) fn 導出のレイアウトを作る(
    device: &ash::Device,
    セットレイアウト: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    作る(device, セットレイアウト, 導出の即時定数バイト数)
}

fn 作る(
    device: &ash::Device,
    セットレイアウト: vk::DescriptorSetLayout,
    即時定数バイト数: u32,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let セット一覧 = [セットレイアウト];
    let 範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(即時定数バイト数)];
    let 生成情報 = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&セット一覧)
        .push_constant_ranges(&範囲一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_pipeline_layout(&生成情報, None)? })
}
