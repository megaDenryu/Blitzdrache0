//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。

mod bind_tally;
mod cloth_types;
mod copy;
mod dispatch;
mod draw_switch_tally;
mod images;
mod point_light_shadow_tally;
mod point_light_shadow_types;
mod post_types;
mod present;
pub(crate) mod record;
mod record_counts;
mod record_metrics;
mod shadow_types;
mod shared_set_bind;
mod sky_types;
mod submit_outcome;
mod submit_present;
mod types;
mod ui_types;

pub(crate) mod acquire;
pub(crate) mod depth_prepass_draw;
pub(crate) mod draw_commands;

pub(crate) use crate::vulkan::descriptor::共有セット束縛;
pub(crate) use acquire::取得結果;
pub(crate) use bind_tally::{セット別束縛計数, セット番号の数};
pub(crate) use cloth_types::{布シャドウ描画入力, 布描画の外部資源, 布描画入力};
pub(crate) use dispatch::{任意描画入力, 同期入力, 描画対象入力, 提示先};
pub(crate) use draw_switch_tally::記録側切替計数;
pub(crate) use images::{フレーム画像一式, 光のにじみ画像};
pub(crate) use point_light_shadow_types::{点光源の影の描画発行, 点光源の影の束縛};
pub(crate) use post_types::{光のにじみ描画入力, 明るさの圧縮描画入力};
pub(crate) use record::記録の実績;
pub use record_counts::記録側の計数;
pub(crate) use record_metrics::記録の計器;
pub(crate) use shadow_types::{シャドウ描画入力, 距離区分別のシャドウ入力};
pub(crate) use sky_types::{空中遠近合成描画入力, 空描画入力};
pub(crate) use submit_outcome::送信後の結末;
pub(crate) use types::{ジオメトリ入力, スキニング描画入力, 描画方式, 粒子描画入力};
pub(crate) use ui_types::{UI描画入力, UI描画項目};

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;

/// 取得済みの画像に対して1フレーム分のコマンドを記録し、送信・提示する。
/// 戻り値の`Err`は送信へ到達しなかったことだけを表し、送信が成った後の失敗は`送信後の結末`に載る。呼び出し元はこの2つを
/// 区別して資源表世代の保持を記録する。もう1つの戻り値は記録の実績
/// (GPUタイムスタンプの「パス名→クエリ開始添字」対応(判断30)と大気のベイク済み画像生成パスの本数)である。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn 描画する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    フレーム構成: &フレーム構成,
    提示先: 提示先<'_>,
    画像一式: &フレーム画像一式<'_>,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    描画対象: 描画対象入力<'_>,
    任意入力: 任意描画入力<'_>,
    描画方式: 描画方式,
    クエリプール: Option<vk::QueryPool>,
    同期: 同期入力,
) -> Result<(送信後の結末, 記録の実績), レンダラーエラー> {
    let 読み戻し待機が必要 = matches!(描画方式, 描画方式::読み戻し { .. });
    let 実績 = record::コマンドを記録する(
        device,
        command_buffer,
        フレーム構成,
        画像一式,
        寸法,
        クリア色,
        描画対象,
        任意入力,
        &描画方式,
        クエリプール,
    )?;
    let 結末 = submit_present::送信して提示する(device, queue, command_buffer, 提示先, &同期, 読み戻し待機が必要)?;
    Ok((結末, 実績))
}
