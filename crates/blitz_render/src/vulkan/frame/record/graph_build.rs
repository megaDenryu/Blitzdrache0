//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。共通画像の登録は`base_images`、
//! ポストプロセス画像の登録は`post_setup`へ分離。

mod base_images;
mod composition;
mod post_passes;
mod post_setup;
mod readback_stage;
mod stages;

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::frame_composition::フレーム構成;
use crate::vulkan::frame::{フレーム画像一式, 任意描画入力, 描画対象入力, 描画方式};
use crate::vulkan::graph;

/// 構築したグラフと、そのフレームで積んだ大気のベイク済み画像生成パスの本数。本数を返すのは、生成の更新判定が意図どおりに
/// 働いているかを実測で見る計器のためである(参照: `crates/blitz_render/src/renderer/pass_tally/atmosphere.rs`)。
pub(super) struct 構築結果<'a> {
    pub(super) グラフ: graph::グラフ<'a>,
    pub(super) 大気のベイク済み画像生成パス数: u32,
    pub(super) 間接照明生成パス数: crate::distant_environment::間接照明生成パス数,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn グラフを構築する<'a>(
    画像一式: &フレーム画像一式,
    フレーム構成: &フレーム構成,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    描画対象: 描画対象入力<'a>,
    任意入力: 任意描画入力<'a>,
    描画方式: &'a 描画方式,
) -> 構築結果<'a> {
    let mut グラフ = graph::グラフ::新規();
    let 基本 = base_images::登録する(&mut グラフ, 画像一式, 寸法);

    // シーン・粒子の描画先: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    let ポスト = post_setup::登録する(&mut グラフ, 画像一式, 任意入力.明るさの圧縮, 任意入力.光のにじみ, 寸法);
    let シーンカラーハンドル = ポスト.as_ref().map_or(基本.スワップチェーン, |構成| 構成.hdrハンドル);

    let 実績 = composition::積む(
        &mut グラフ,
        フレーム構成,
        画像一式,
        &基本,
        ポスト.as_ref(),
        シーンカラーハンドル,
        クリア色,
        描画対象,
        任意入力,
        描画方式,
        寸法,
    );

    グラフ.最終用途を宣言する(基本.スワップチェーン, graph::画像用途::提示);
    構築結果 {
        グラフ,
        大気のベイク済み画像生成パス数: 実績.大気のベイク済み画像生成パス数,
        間接照明生成パス数: 実績.間接照明生成パス数,
    }
}
