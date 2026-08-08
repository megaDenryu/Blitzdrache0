//! 動きベクトル画像・今のフレームの色の画像・履歴画像をグラフへ登録するときの初期状態。
//!
//! 前の2枚を焼いた画像や局所可視度と別に持つのは、この2枚がフレームをまたいで内容を保たない点が違うためである。
//! シーン描画のパスが毎フレーム消去してから書くため、レイアウトはUNDEFINEDでよい。
//!
//! 履歴画像だけは中身を保つ。同じ2枚が読み側と書き側を交互に入れ替わるため、前のフレームで最後に起きたことは
//! 画像ごとに違う。初期状態を1つの値で表すために、休むレイアウトをGENERALに固定し、段とアクセスは
//! 「カラー添付への書き込み」と「画素段の標本読み」の和で表す。

use ash::vk;

use crate::vulkan::graph::state::画像状態;

/// 動きベクトル画像の、前フレーム「カラー添付への書き込み」直後を想定した状態。
///
/// 注意: グラフ内で最後にこの画像を書くパスは、空を積むフレームでは空、積まないフレームではシーン描画である。
/// どちらもカラー添付への書き込みであるため、段とアクセスはこの1つの値で表せる。
pub(crate) fn 前フレーム動きベクトル書き込み直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
        vk::ImageLayout::UNDEFINED,
    )
}

/// 今のフレームの色の画像の、前フレーム「時間再構成パスの画素段読み」直後を想定した状態。
///
/// 注意: グラフ内で最後にこの画像を使うパスは常に時間再構成(画素段読み)のため、次フレーム冒頭の
/// シーン描画(カラー書き)との間のWARハザードをこの値で表現する。layoutはUNDEFINEDにして
/// 内容を保持しない(本フレームでCLEARし直すため)。
pub(crate) fn 前フレーム今のフレームの色読み直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::UNDEFINED,
    )
}

/// 履歴画像2枚に共通の、前フレーム直後を想定した状態。
///
/// 注意: レイアウトはGENERALである。2枚はフレームをまたいで中身を保ち、確保直後に零で埋めた値も
/// このレイアウトで置かれている(`vulkan/temporal_reconstruction/fill.rs`)。UNDEFINEDにするとその値を捨てられる。
pub(crate) fn 履歴画像の前フレーム直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE | vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::GENERAL,
    )
}
