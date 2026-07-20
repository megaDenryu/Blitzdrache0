//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。共通画像の登録は`base_images`、
//! ポストプロセス画像の登録は`post_setup`へ分離。

mod base_images;
mod post_passes;
mod post_setup;

use ash::vk;

use super::{particle_draw_pass, particle_update_pass, readback_pass, scene_pass, shadow_pass, ui_pass};
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{
    シャドウ描画入力, ジオメトリ入力, トーンマップ描画入力, ブルーム描画入力, フレーム画像一式, 描画方式,
    粒子描画入力, UI描画入力,
};
use crate::vulkan::graph;

#[allow(clippy::too_many_arguments)]
pub(super) fn グラフを構築する<'a>(
    画像一式: &フレーム画像一式,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &'a ジオメトリ入力,
    シャドウ入力: &'a シャドウ描画入力,
    粒子入力: Option<&'a 粒子描画入力>,
    ブルーム入力: Option<&'a ブルーム描画入力>,
    トーンマップ入力: Option<&'a トーンマップ描画入力>,
    ui入力: Option<&'a UI描画入力>,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規();
    let 基本 = base_images::登録する(&mut グラフ, 画像一式, 寸法);

    // シーン・粒子の描画先: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    let ポスト = post_setup::登録する(&mut グラフ, 画像一式, トーンマップ入力, ブルーム入力, 寸法);
    let シーンカラーハンドル = ポスト.as_ref().map_or(基本.スワップチェーン, |構成| 構成.hdrハンドル);

    // 実行順序=宣言順(判断27)。シャドウパスの深度書きをシーン描画の読みより先に積む(M6)。
    グラフ.パスを積む(shadow_pass::作る(基本.シャドウマップ, シャドウ入力));

    グラフ.パスを積む(scene_pass::作る(
        シーンカラーハンドル,
        基本.深度,
        基本.シャドウマップ,
        クリア色,
        pipeline,
        ジオメトリ入力,
        寸法,
    ));

    if let Some(粒子入力) = 粒子入力 {
        let 粒子ハンドル = グラフ.バッファを登録する(粒子入力.バッファ, graph::前フレーム粒子読み直後状態());
        グラフ.パスを積む(particle_update_pass::作る(粒子ハンドル, 粒子入力));
        グラフ.パスを積む(particle_draw_pass::作る(シーンカラーハンドル, 基本.深度, 粒子ハンドル, 粒子入力, 寸法));
    }

    // ポスト列(判断41)の積み込みはpost_passesへ委ねる。
    if let Some(構成) = &ポスト {
        post_passes::積む(&mut グラフ, 構成, 基本.スワップチェーン, 寸法);
    }

    if let Some(ui入力) = ui入力 {
        グラフ.パスを積む(ui_pass::作る(基本.スワップチェーン, ui入力, 寸法));
    }

    if let 描画方式::読み戻し { バッファ } = 描画方式 {
        グラフ.パスを積む(readback_pass::作る(基本.スワップチェーン, *バッファ, 寸法));
    }

    グラフ.最終用途を宣言する(基本.スワップチェーン, graph::画像用途::提示);
    グラフ
}
