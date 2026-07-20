//! スワップチェーン確定後、主要資源(ジオメトリ・テクスチャ・パイプライン等)に
//! 続けて組み立てる残り: 粒子トイ(任意)・GPU計測(対応環境のみ)・開発用UI(常設)。
//! `generate_resources`の行数分割のためだけに切り出した内部ヘルパー。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::particle_shader_set::粒子シェーダー一式;
use crate::shader_set::シェーダー一式;
use crate::vulkan;

pub(super) struct 追加資源 {
    pub(super) 粒子: Option<vulkan::particles::粒子リソース一式>,
    pub(super) gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    pub(super) ui一式: vulkan::ui::UIリソース一式,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &vulkan::transfer::転送実行環境,
    swapchain: &vulkan::swapchain::スワップチェーン,
    深度形式: vk::Format,
    ユニフォーム: &vulkan::uniform::フレームユニフォーム一式,
    粒子シェーダー: Option<&粒子シェーダー一式>,
    uiシェーダー: &シェーダー一式,
    タイムスタンプ対応か: bool,
    タイムスタンプ周期ns: f32,
) -> Result<追加資源, レンダラーエラー> {
    let 粒子 = 粒子シェーダー
        .map(|シェーダー| {
            vulkan::particles::粒子リソース一式::生成する(
                device,
                メモリプロパティ,
                転送環境,
                swapchain.画像形式,
                深度形式,
                ユニフォーム,
                シェーダー,
            )
        })
        .transpose()?;

    let gpu計測 =
        vulkan::gpu_timing::パス別GPU計測::生成する(device, タイムスタンプ対応か, タイムスタンプ周期ns)?;

    let ui一式 = vulkan::ui::UIリソース一式::生成する(device, swapchain.画像形式, uiシェーダー)?;

    Ok(追加資源 { 粒子, gpu計測, ui一式 })
}
