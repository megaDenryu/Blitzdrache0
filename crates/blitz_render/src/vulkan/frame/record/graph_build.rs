//! フレーム1回ぶんのレンダーグラフ構築。画像・バッファの登録とパスの積み上げのみを行い、
//! 実行(バリア導出・記録)は`graph::実行する`に委ねる。

use ash::vk;

use super::{particle_draw_pass, particle_update_pass, readback_pass, scene_pass};
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{ジオメトリ入力, 描画方式, 粒子描画入力};
use crate::vulkan::graph;

#[allow(clippy::too_many_arguments)]
pub(super) fn グラフを構築する<'a>(
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &'a ジオメトリ入力,
    粒子入力: Option<&'a 粒子描画入力>,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規(寸法);
    let カラーハンドル = グラフ.画像を登録する(
        画像,
        画像ビュー,
        graph::画像アスペクト::カラー,
        graph::取得直後の色画像状態(),
    );
    let 深度ハンドル = グラフ.画像を登録する(
        深度画像,
        深度画像ビュー,
        graph::画像アスペクト::深度,
        graph::前フレーム深度書き込み直後状態(),
    );

    グラフ.パスを積む(scene_pass::作る(
        カラーハンドル,
        深度ハンドル,
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

    if let 描画方式::読み戻し { バッファ } = 描画方式 {
        グラフ.パスを積む(readback_pass::作る(カラーハンドル, *バッファ, 寸法));
    }

    グラフ.最終用途を宣言する(カラーハンドル, graph::画像用途::提示);
    グラフ
}
