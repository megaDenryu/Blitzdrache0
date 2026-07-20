//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。共通画像の登録は`base_images`へ分離。

mod base_images;

use ash::vk;

use super::{particle_draw_pass, particle_update_pass, readback_pass, scene_pass, shadow_pass, tonemap_pass, ui_pass};
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{
    シャドウ描画入力, ジオメトリ入力, トーンマップ描画入力, フレーム画像一式, 描画方式, 粒子描画入力, UI描画入力,
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
    トーンマップ入力: Option<&'a トーンマップ描画入力>,
    ui入力: Option<&'a UI描画入力>,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規();
    let 基本 = base_images::登録する(&mut グラフ, 画像一式, 寸法);

    // シーン・粒子の描画先: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    // トーンマップ入力とHDR画像の有無は常に一致する(レンダラーが対で生成する)。
    let トーンマップ構成 = match (トーンマップ入力, 画像一式.hdr) {
        (Some(入力), Some((hdr画像, hdrビュー))) => {
            let hdrハンドル = グラフ.画像を登録する(
                hdr画像,
                hdrビュー,
                graph::画像アスペクト::カラー,
                graph::前フレームhdr読み直後状態(),
                寸法,
            );
            Some((hdrハンドル, 入力))
        }
        (None, None) => None,
        _ => panic!("トーンマップ入力とHDR画像の有無が一致しない(レンダラーの配線のバグ)"),
    };
    let シーンカラーハンドル = トーンマップ構成.map_or(基本.スワップチェーン, |(hdrハンドル, _)| hdrハンドル);

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

    if let Some((hdrハンドル, 入力)) = トーンマップ構成 {
        グラフ.パスを積む(tonemap_pass::作る(hdrハンドル, 基本.スワップチェーン, 入力, 寸法));
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
