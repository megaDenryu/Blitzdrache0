//! テクスチャの生成局面。呼ばれるのは資源表の世代がテクスチャ1枚をGPUへ常駐させるときである。
//!
//! 縮小段の積み方には2系統ある。非圧縮のRGBA8は原寸1枚だけを転送して残りの段をGPUのblitで作り、
//! ブロック圧縮は全段をファイルから受け取ってbufferからimageへのコピーで積む。GPU側にブロック圧縮の符号化器が
//! 存在せず、圧縮形式の画像を書き込み先とするblitが成立しないためである
//! (参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「判断d」)。
//! どちらを採るかの判定は`縮小段の積み方を選ぶ`の1箇所だけで行い、以降の工程は選んだ積み方に従う。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::texture_material::level_extent::縮小段数の上限を求める;
use crate::texture_material::{テクスチャ格納形式, テクスチャ素材};
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::transfer::ステージング経由の転送係;

use super::{format_support, image, upload, view, テクスチャ, 画像を破棄する};

/// 縮小段をどう用意するかの2系統。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 縮小段の積み方 {
    原寸の転送とGPUのblit, // 原寸1枚を転送し、残りの段をGPUのblitの連鎖で作る。
    全段の転送,            // ファイルが持つ全段をbufferからimageへのコピーで積む。
}

pub(super) fn テクスチャを生成する(
    転送係: ステージング経由の転送係<'_>,
    問い合わせ: 物理デバイス問い合わせ<'_>,
    素材: &テクスチャ素材,
) -> Result<テクスチャ, レンダラーエラー> {
    let 確保係 = 転送係.確保係();
    let device = 確保係.論理デバイス();
    let 形式 = format_support::vulkan形式を選ぶ(素材.用途(), 素材.格納形式())?;
    let 積み方 = 縮小段の積み方を選ぶ(素材.格納形式());
    機材の対応を確認する(問い合わせ, 形式, 積み方)?;

    let 縮小段数 = 縮小段数を求める(素材, 積み方);
    let (image, memory) = image::生成する(確保係, 素材.幅(), 素材.高さ(), 縮小段数, 形式, 画像の使い道(積み方))?;

    if let Err(誤り) = upload::素材の縮小段を画像へ転送する(転送係, image, 素材, 縮小段数, 積み方) {
        画像を破棄する(device, image, memory);
        return Err(誤り);
    }

    let image_view = match view::画像ビューを作る(確保係, image, 縮小段数, 形式) {
        Ok(view) => view,
        Err(誤り) => {
            画像を破棄する(device, image, memory);
            return Err(誤り);
        }
    };
    Ok(テクスチャ::部品から組み立てる(image, memory, image_view))
}

fn 縮小段の積み方を選ぶ(格納形式: テクスチャ格納形式) -> 縮小段の積み方 {
    if 格納形式.縮小段をファイルが運ぶか() {
        縮小段の積み方::全段の転送
    } else {
        縮小段の積み方::原寸の転送とGPUのblit
    }
}

/// 積み方ごとに要る機能だけを問い合わせる。ブロック圧縮の経路からblitの確認を呼ぶと、blitを1度も使わないのに
/// blitへ対応しない機材を拒むことになる。
fn 機材の対応を確認する(
    問い合わせ: 物理デバイス問い合わせ<'_>,
    形式: vk::Format,
    積み方: 縮小段の積み方,
) -> Result<(), レンダラーエラー> {
    match 積み方 {
        縮小段の積み方::原寸の転送とGPUのblit => format_support::blitフィルタ対応を確認する(問い合わせ, 形式),
        縮小段の積み方::全段の転送 => {
            format_support::ブロック圧縮の標本化と転送先の対応を確認する(問い合わせ, 形式)
        }
    }
}

/// 縮小段数の出どころは積み方で変わる。GPUのblitで作る側は原寸から作れる本数を計算し、
/// ファイルから転送する側は素材が運ぶ段の本数をそのまま使う。
fn 縮小段数を求める(素材: &テクスチャ素材, 積み方: 縮小段の積み方) -> u32 {
    match 積み方 {
        縮小段の積み方::原寸の転送とGPUのblit => 縮小段数の上限を求める(素材.幅(), 素材.高さ()),
        縮小段の積み方::全段の転送 => 素材.縮小段数(),
    }
}

/// blitの元になるのは縮小段をGPUで作る画像だけである。使わない用途を立てると、その用途に対応しない形式で画像を作れない。
fn 画像の使い道(積み方: 縮小段の積み方) -> vk::ImageUsageFlags {
    let 共通 = vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
    match 積み方 {
        縮小段の積み方::原寸の転送とGPUのblit => 共通 | vk::ImageUsageFlags::TRANSFER_SRC,
        縮小段の積み方::全段の転送 => 共通,
    }
}
