//! ポストプロセス資源(HDR中間画像+トーンマップ一式)の組み立て(判断38・39)。
//! `ポスト処理有効`がfalseならどちらもNone(シーンがスワップチェーンへ直接描く現行構成)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan;

pub(super) struct ポスト資源 {
    pub(super) hdrターゲット: Option<vulkan::hdr_target::HDRターゲット>,
    pub(super) トーンマップ: Option<vulkan::tonemap::トーンマップ一式>,
}

pub(super) fn 組み立てる(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    swapchain: &vulkan::swapchain::スワップチェーン,
    トーンマップシェーダー: &シェーダー一式,
    ポスト処理有効: bool,
) -> Result<ポスト資源, レンダラーエラー> {
    if !ポスト処理有効 {
        return Ok(ポスト資源 { hdrターゲット: None, トーンマップ: None });
    }

    let hdrターゲット = vulkan::hdr_target::HDRターゲット::生成する(device, メモリプロパティ, swapchain.寸法)?;
    let トーンマップ = match vulkan::tonemap::トーンマップ一式::生成する(
        device,
        swapchain.画像形式,
        トーンマップシェーダー,
        hdrターゲット.画像ビュー,
    ) {
        Ok(一式) => 一式,
        Err(誤り) => {
            hdrターゲット.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(ポスト資源 { hdrターゲット: Some(hdrターゲット), トーンマップ: Some(トーンマップ) })
}
