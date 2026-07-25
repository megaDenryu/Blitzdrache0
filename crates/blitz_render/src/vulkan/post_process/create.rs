//! ポスト処理一式の生成手順(判断38・39・41)。途中失敗時は生成済みの資源を逆順で片付ける。
//! 寸法に連動する画像の生成だけは`resize`からも呼ぶため、独立した関数に分けてある。

use ash::vk;

use super::ポスト処理一式;
use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::bloom::ブルーム一式;
use crate::vulkan::bloom_targets::ブルームピラミッド;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tonemap::トーンマップ一式;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    スワップチェーン画像形式: vk::Format,
    寸法: vk::Extent2D,
    シェーダー: &シェーダー束,
) -> Result<ポスト処理一式, レンダラーエラー> {
    let (hdrターゲット, ブルームピラミッド) = 画像を生成する(device, メモリプロパティ, 寸法)?;
    match パイプラインを生成する(device, スワップチェーン画像形式, シェーダー, &hdrターゲット, &ブルームピラミッド)
    {
        Ok((ブルーム, トーンマップ)) => Ok(ポスト処理一式 {
            hdrターゲット,
            ブルームピラミッド,
            ブルーム,
            トーンマップ,
        }),
        Err(誤り) => {
            ブルームピラミッド.破棄する(device);
            hdrターゲット.破棄する(device);
            Err(誤り)
        }
    }
}

/// スワップチェーン寸法に連動する画像2つ。ピラミッドの生成に失敗したらHDR中間画像を片付ける。
pub(super) fn 画像を生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    寸法: vk::Extent2D,
) -> Result<(HDRターゲット, ブルームピラミッド), レンダラーエラー> {
    let hdr = HDRターゲット::生成する(device, メモリプロパティ, 寸法)?;
    match ブルームピラミッド::生成する(device, メモリプロパティ, 寸法) {
        Ok(ピラミッド) => Ok((hdr, ピラミッド)),
        Err(誤り) => {
            hdr.破棄する(device);
            Err(誤り)
        }
    }
}

/// 寸法に依存しないパイプラインとサンプラーの部分。生成直後の画像ビュー束縛も各一式の生成が行う。
fn パイプラインを生成する(
    device: &GPUデバイス,
    スワップチェーン画像形式: vk::Format,
    シェーダー: &シェーダー束,
    hdr: &HDRターゲット,
    ピラミッド: &ブルームピラミッド,
) -> Result<(ブルーム一式, トーンマップ一式), レンダラーエラー> {
    let ブルーム = ブルーム一式::生成する(
        device,
        &シェーダー.ブルーム前処理,
        &シェーダー.ブルーム縮小,
        &シェーダー.ブルーム拡大,
        hdr.画像ビュー,
        ピラミッド,
    )?;
    let トーンマップ = トーンマップ一式::生成する(
        device,
        スワップチェーン画像形式,
        &シェーダー.トーンマップ,
        hdr.画像ビュー,
        ピラミッド.最終ビュー(),
    );
    match トーンマップ {
        Ok(一式) => Ok((ブルーム, 一式)),
        Err(誤り) => {
            ブルーム.破棄する(device);
            Err(誤り)
        }
    }
}
