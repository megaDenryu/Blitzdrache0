//! ポストプロセス資源(HDR中間画像+ブルーム画像2枚+ブルーム/トーンマップ一式)の
//! 組み立て(判断38・39)。`ポスト処理有効`がfalseならすべてNone(シーンがスワップチェーンへ直接描く構成)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct ポスト資源 {
    pub(super) hdrターゲット: Option<vulkan::hdr_target::HDRターゲット>,
    pub(super) ブルームピラミッド: Option<vulkan::bloom_targets::ブルームピラミッド>,
    pub(super) ブルーム: Option<vulkan::bloom::ブルーム一式>,
    pub(super) トーンマップ: Option<vulkan::tonemap::トーンマップ一式>,
}

pub(super) fn 組み立てる(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    swapchain: &vulkan::swapchain::スワップチェーン,
    シェーダー: &シェーダー束,
    ポスト処理有効: bool,
) -> Result<ポスト資源, レンダラーエラー> {
    if !ポスト処理有効 {
        return Ok(ポスト資源 {
            hdrターゲット: None,
            ブルームピラミッド: None,
            ブルーム: None,
            トーンマップ: None,
        });
    }

    let hdr = vulkan::hdr_target::HDRターゲット::生成する(device, メモリプロパティ, swapchain.寸法)?;
    let ピラミッド = match vulkan::bloom_targets::ブルームピラミッド::生成する(device, メモリプロパティ, swapchain.寸法) {
        Ok(一式) => 一式,
        Err(誤り) => {
            hdr.破棄する(device);
            return Err(誤り);
        }
    };
    let ブルーム = match vulkan::bloom::ブルーム一式::生成する(
        device,
        &シェーダー.ブルーム前処理,
        &シェーダー.ブルーム縮小,
        &シェーダー.ブルーム拡大,
        hdr.画像ビュー,
        &ピラミッド,
    ) {
        Ok(一式) => 一式,
        Err(誤り) => {
            ピラミッド.破棄する(device);
            hdr.破棄する(device);
            return Err(誤り);
        }
    };
    let トーンマップ = match vulkan::tonemap::トーンマップ一式::生成する(
        device,
        swapchain.画像形式,
        &シェーダー.トーンマップ,
        hdr.画像ビュー,
        ピラミッド.最終ビュー(),
    ) {
        Ok(一式) => 一式,
        Err(誤り) => {
            ブルーム.破棄する(device);
            ピラミッド.破棄する(device);
            hdr.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(ポスト資源 {
        hdrターゲット: Some(hdr),
        ブルームピラミッド: Some(ピラミッド),
        ブルーム: Some(ブルーム),
        トーンマップ: Some(トーンマップ),
    })
}
