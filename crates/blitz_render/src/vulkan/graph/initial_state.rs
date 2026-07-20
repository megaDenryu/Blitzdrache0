//! 外部持ち込み画像をグラフへ登録する際の初期状態。用途写像(`usage::image_usage_mapping`)
//! とは別に扱う: どちらも「取得直後・前フレーム直後で今何も起きていない」状態であり、
//! パス内で実際に使われる用途(カラー出力・提示等)とは異なる固有の値だから。

use ash::vk;

use super::state::画像状態;
use super::usage::image_usage_mapping::深度書き込み段;

/// スワップチェーンから取得した直後のカラー画像の状態。
///
/// 注意: srcStageMaskはTOP_OF_PIPEではなくCOLOR_ATTACHMENT_OUTPUTにする。
/// 取得セマフォの待機はCOLOR_ATTACHMENT_OUTPUT段で行うため、最初のバリアがそれより
/// 早い段だと待機のスコープ外になり、vkAcquireNextImageKHRの読み出しに対して
/// WRITE_AFTER_READ検証エラーになる。layoutはUNDEFINEDのままでよい
/// （このエンジンは常に全面クリアするため前回内容を保持する必要がない）。
pub(crate) fn 取得直後の色画像状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        vk::AccessFlags2::empty(),
        vk::ImageLayout::UNDEFINED,
    )
}

/// 深度画像の、前フレーム書き込み直後を想定した状態。
///
/// 注意: フレームインフライト2枚で単一の深度画像を共有するため、前フレームの
/// 深度書き込みとのWAWハザードを安全化するstage/accessを保つ。layoutはUNDEFINEDにして
/// 内容を保持しない(本フレームでCLEARし直すため)。
pub(crate) fn 前フレーム深度書き込み直後状態() -> 画像状態 {
    画像状態::生成する(
        深度書き込み段(),
        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
        vk::ImageLayout::UNDEFINED,
    )
}
