//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。

mod cloth_types;
mod copy;
mod dispatch;
mod images;
mod record;
mod shadow_types;
mod sky_types;
mod submit_present;
mod types;
mod ui_types;

pub(crate) mod acquire;
pub(crate) mod draw_commands;

pub(crate) use acquire::取得結果;
pub(crate) use cloth_types::{布シャドウ描画入力, 布描画の外部資源, 布描画入力};
pub(crate) use dispatch::{任意描画入力, 同期入力, 描画対象入力, 提示先};
pub(crate) use images::{フレーム画像一式, ブルーム画像};
pub(crate) use record::記録の実績;
pub(crate) use shadow_types::{シャドウ描画入力, 距離区分別のシャドウ入力};
pub(crate) use sky_types::{空中遠近合成描画入力, 空描画入力};
pub(crate) use types::{
    ジオメトリ入力, スキニング描画入力, トーンマップ描画入力, ブルーム描画入力, 描画方式, 粒子描画入力
};
pub(crate) use ui_types::{UI描画入力, UI描画項目};

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;

/// 取得済みの画像に対して1フレーム分のコマンドを記録し、送信・提示する。
/// 戻り値は「提示まで到達したか（true）／スワップチェーンが陳腐化していたか（false）」と、記録の実績
/// (GPUタイムスタンプの「パス名→クエリ開始添字」対応(判断30)と大気のベイク済み画像生成パスの本数)。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn 描画する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    フレーム構成: &フレーム構成,
    提示先: 提示先<'_>,
    画像一式: &フレーム画像一式,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    描画対象: 描画対象入力<'_>,
    任意入力: 任意描画入力<'_>,
    描画方式: 描画方式,
    クエリプール: Option<vk::QueryPool>,
    同期: 同期入力,
) -> Result<(bool, 記録の実績), レンダラーエラー> {
    let 読み戻し待機が必要 = matches!(描画方式, 描画方式::読み戻し { .. });
    let 実績 = record::コマンドを記録する(
        device,
        command_buffer,
        フレーム構成,
        画像一式,
        寸法,
        クリア色,
        pipeline,
        描画対象,
        任意入力,
        &描画方式,
        クエリプール,
    )?;
    let 提示劣化 = dispatch::送信して提示する(device, queue, command_buffer, 提示先, 同期, 読み戻し待機が必要)?;
    Ok((提示劣化, 実績))
}
