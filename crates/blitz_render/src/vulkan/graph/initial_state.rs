//! 外部持ち込み画像・バッファをグラフへ登録する際の初期状態。用途写像(`usage::image_usage_mapping`等)とは別に扱う: どちらも「取得直後・前フレーム直後で今何も起きていない」状態であり、パス内で実際に使われる用途(カラー出力・提示等)とは異なる固有の値だから。
//! 焼いた画像(大気のベイク済み画像・遠方環境とその派生表現)の初期状態だけは、焼き直す場合と参照するだけの場合で1つの不変条件の表裏になるため`baked_image`が持つ。局所可視度の2枚は記憶画像としても読まれるため`local_visibility`が別に持つ。

mod baked_image;
mod local_visibility;
mod temporal_reconstruction;

pub(crate) use baked_image::{焼いた画像を参照するだけのときの初期状態, 焼いた画像を焼き直すときの初期状態};
pub(crate) use local_visibility::局所可視度の画像の前フレーム直後状態;
pub(crate) use temporal_reconstruction::{
    前フレーム今のフレームの色読み直後状態, 前フレーム動きベクトル書き込み直後状態, 履歴画像の前フレーム直後状態,
};

use ash::vk;

use super::buffer_state::バッファ状態;
use super::state::画像状態;
use super::usage::image_usage_mapping::深度書き込み段;

/// スワップチェーンから取得した直後のカラー画像の状態。
///
/// 注意: srcStageMaskはTOP_OF_PIPEではなくCOLOR_ATTACHMENT_OUTPUTにする。取得セマフォの待機はCOLOR_ATTACHMENT_OUTPUT段で行うため、最初のバリアがそれより早い段だと待機のスコープ外になり、vkAcquireNextImageKHRの読み出しに対してWRITE_AFTER_READ検証エラーになる。
/// layoutはUNDEFINEDのままでよい（このエンジンは常に全面クリアするため前回内容を保持する必要がない）。
pub(crate) fn 取得直後の色画像状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        vk::AccessFlags2::empty(),
        vk::ImageLayout::UNDEFINED,
    )
}

/// 深度画像の、前フレーム書き込み直後を想定した状態。
///
/// 注意: 進行中フレーム2枚で単一の深度画像を共有するため、前フレームの深度書き込みとのWAWハザードを安全化するstage/accessを保つ。layoutはUNDEFINEDにして内容を保持しない(本フレームでCLEARし直すため)。
pub(crate) fn 前フレーム深度書き込み直後状態() -> 画像状態 {
    画像状態::生成する(
        深度書き込み段(),
        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
        vk::ImageLayout::UNDEFINED,
    )
}

/// シャドウマップの、前フレーム「画素段シェーダー読み」直後を想定した状態。
///
/// 注意: シャドウマップはグラフの外(レンダラー)で毎フレーム使い回す固定サイズの
/// 単一画像(判断35)。グラフ内で最後にこの画像を使うパスは常にシーン描画(判断35の
/// シャドウ読み)のため、次フレーム冒頭のシャドウパス(深度書き)との間のWARハザードを
/// この値で表現する。layoutはUNDEFINEDにして内容を保持しない(本フレームでCLEARし直すため)。
pub(crate) fn 前フレームシャドウマップ読み直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::UNDEFINED,
    )
}

/// HDR中間画像の、前フレーム「明るさの圧縮パスの画素段読み」直後を想定した状態(判断38)。
///
/// 注意: HDR画像はグラフの外(レンダラー)で毎フレーム使い回す。グラフ内で最後に
/// この画像を使うパスは常に明るさの圧縮(画素段読み)のため、次フレーム冒頭の
/// シーン描画(カラー書き)との間のWARハザードをこの値で表現する。layoutはUNDEFINEDにして
/// 内容を保持しない(本フレームでCLEARし直すため)。
pub(crate) fn 前フレームhdr読み直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::UNDEFINED,
    )
}

/// 粒子ストレージバッファの、前フレーム「頂点段シェーダー読み」直後を想定した状態。
///
/// 注意: 粒子バッファはグラフの外(レンダラー)で毎フレーム使い回す(1回だけ確保)。
/// グラフ内で最後にこのバッファを使うパスは常に粒子描画(頂点段シェーダー読み)のため、
/// 次フレーム冒頭の粒子更新(コンピュート書き)との間のWARハザードをこの値で表現する。
pub(crate) fn 前フレーム粒子読み直後状態() -> バッファ状態 {
    バッファ状態::生成する(vk::PipelineStageFlags2::VERTEX_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
}

/// 布シミュの中間バッファ(粒子・前位置・空間グリッド)の、前フレーム「コンピュート読み」直後を想定した状態(判断54)。
/// グラフ内で最後にこれらを使うパスはコンピュート読み(頂点生成・分離・仕上げ)のため。
pub(crate) fn 前フレームコンピュート読み直後状態() -> バッファ状態 {
    バッファ状態::生成する(vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
}

/// 自動露出の露出状態バッファの、前フレーム「明るさの圧縮の画素段ストレージ読み」直後を想定した状態。
/// 注意: グラフ内で最後にこのバッファを使うパスは常に明るさの圧縮(画素段の読み)のため、次フレームの
/// 導出と適応(コンピュート書き)との間のWARハザードをこの値で表現する。
pub(crate) fn 前フレーム画素段読み直後状態() -> バッファ状態 {
    バッファ状態::生成する(vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
}

/// スキン済み頂点バッファの、前フレーム「頂点入力読み」直後を想定した状態(判断44)。
///
/// 注意: グラフ内で最後にこのバッファを使うパスは常にシーン描画(頂点入力読み)のため、
/// 次フレーム冒頭のスキニング(コンピュート書き)との間のWARハザードをこの値で表現する。
pub(crate) fn 前フレーム頂点入力読み直後状態() -> バッファ状態 {
    バッファ状態::生成する(vk::PipelineStageFlags2::VERTEX_INPUT, vk::AccessFlags2::VERTEX_ATTRIBUTE_READ)
}
