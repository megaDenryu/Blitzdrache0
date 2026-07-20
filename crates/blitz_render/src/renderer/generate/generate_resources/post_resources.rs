//! ポストプロセス資源(HDR中間画像+ブルーム画像2枚+ブルーム/トーンマップ一式)の
//! 組み立て(判断38・39)。`ポスト処理有効`がfalseならすべてNone(シーンがスワップチェーンへ直接描く構成)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;

pub(super) struct ポスト資源 {
    pub(super) hdrターゲット: Option<vulkan::hdr_target::HDRターゲット>,
    pub(super) ブルームターゲット: Option<vulkan::bloom_targets::ブルームターゲット>,
    pub(super) ブルーム: Option<vulkan::bloom::ブルーム一式>,
    pub(super) トーンマップ: Option<vulkan::tonemap::トーンマップ一式>,
}

pub(super) fn 組み立てる(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    swapchain: &vulkan::swapchain::スワップチェーン,
    シェーダー: &シェーダー束,
    ポスト処理有効: bool,
) -> Result<ポスト資源, レンダラーエラー> {
    if !ポスト処理有効 {
        return Ok(ポスト資源 { hdrターゲット: None, ブルームターゲット: None, ブルーム: None, トーンマップ: None });
    }

    let hdr = vulkan::hdr_target::HDRターゲット::生成する(device, メモリプロパティ, swapchain.寸法)?;
    let ブルームターゲット =
        match vulkan::bloom_targets::ブルームターゲット::生成する(device, メモリプロパティ, swapchain.寸法) {
            Ok(一式) => 一式,
            Err(誤り) => {
                hdr.破棄する(device);
                return Err(誤り);
            }
        };
    let ブルーム = match vulkan::bloom::ブルーム一式::生成する(
        device,
        &シェーダー.ブルーム抽出,
        &シェーダー.ブルームぼかし,
        hdr.画像ビュー,
        ブルームターゲット.a.画像ビュー,
        ブルームターゲット.b.画像ビュー,
    ) {
        Ok(一式) => 一式,
        Err(誤り) => {
            ブルームターゲット.破棄する(device);
            hdr.破棄する(device);
            return Err(誤り);
        }
    };
    let トーンマップ = match vulkan::tonemap::トーンマップ一式::生成する(
        device,
        swapchain.画像形式,
        &シェーダー.トーンマップ,
        hdr.画像ビュー,
        ブルームターゲット.a.画像ビュー,
    ) {
        Ok(一式) => 一式,
        Err(誤り) => {
            ブルーム.破棄する(device);
            ブルームターゲット.破棄する(device);
            hdr.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(ポスト資源 {
        hdrターゲット: Some(hdr),
        ブルームターゲット: Some(ブルームターゲット),
        ブルーム: Some(ブルーム),
        トーンマップ: Some(トーンマップ),
    })
}
