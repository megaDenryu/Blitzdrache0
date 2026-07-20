//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。

use ash::vk;

use super::{particle_draw_pass, particle_update_pass, readback_pass, scene_pass, shadow_pass, ui_pass};
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力, 描画方式, 粒子描画入力, UI描画入力};
use crate::vulkan::graph;

#[allow(clippy::too_many_arguments)]
pub(super) fn グラフを構築する<'a>(
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    シャドウマップ画像: vk::Image,
    シャドウマップ画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &'a ジオメトリ入力,
    シャドウ入力: &'a シャドウ描画入力,
    粒子入力: Option<&'a 粒子描画入力>,
    ui入力: Option<&'a UI描画入力>,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規();
    let カラーハンドル = グラフ.画像を登録する(
        画像,
        画像ビュー,
        graph::画像アスペクト::カラー,
        graph::取得直後の色画像状態(),
        寸法,
    );
    let 深度ハンドル = グラフ.画像を登録する(
        深度画像,
        深度画像ビュー,
        graph::画像アスペクト::深度,
        graph::前フレーム深度書き込み直後状態(),
        寸法,
    );
    let シャドウマップ寸法 = vk::Extent2D {
        width: crate::vulkan::shadow_map::シャドウマップ一辺,
        height: crate::vulkan::shadow_map::シャドウマップ一辺,
    };
    let シャドウマップハンドル = グラフ.画像を登録する(
        シャドウマップ画像,
        シャドウマップ画像ビュー,
        graph::画像アスペクト::深度,
        graph::前フレームシャドウマップ読み直後状態(),
        シャドウマップ寸法,
    );

    // 実行順序=宣言順(判断27)。シャドウパスの深度書きをシーン描画の読みより
    // 先に積むことで、この依存がバリア導出で表現される(M6 DoDの中心)。
    グラフ.パスを積む(shadow_pass::作る(シャドウマップハンドル, シャドウ入力));

    グラフ.パスを積む(scene_pass::作る(
        カラーハンドル,
        深度ハンドル,
        シャドウマップハンドル,
        クリア色,
        pipeline,
        ジオメトリ入力,
        寸法,
    ));

    if let Some(粒子入力) = 粒子入力 {
        let 粒子ハンドル =
            グラフ.バッファを登録する(粒子入力.バッファ, graph::前フレーム粒子読み直後状態());
        グラフ.パスを積む(particle_update_pass::作る(粒子ハンドル, 粒子入力));
        グラフ.パスを積む(particle_draw_pass::作る(カラーハンドル, 深度ハンドル, 粒子ハンドル, 粒子入力, 寸法));
    }

    if let Some(ui入力) = ui入力 {
        グラフ.パスを積む(ui_pass::作る(カラーハンドル, ui入力, 寸法));
    }

    if let 描画方式::読み戻し { バッファ } = 描画方式 {
        グラフ.パスを積む(readback_pass::作る(カラーハンドル, *バッファ, 寸法));
    }

    グラフ.最終用途を宣言する(カラーハンドル, graph::画像用途::提示);
    グラフ
}
