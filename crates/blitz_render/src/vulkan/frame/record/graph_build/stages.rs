//! レンダーグラフへシミュレーション、シーン描画、後処理を依存順に積む。

use ash::vk;

use super::{base_images, post_passes, post_setup};
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::record::{
    cloth_passes, particle_draw_pass, particle_update_pass, readback_pass, scene_pass, shadow_pass, skinning_pass, ui_pass,
};
use crate::vulkan::frame::{
    UI描画入力, シャドウ描画入力, ジオメトリ入力, スキニング描画入力, 布描画入力, 描画方式, 粒子描画入力
};
use crate::vulkan::graph;

pub(super) fn シミュレーションを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    スキニング入力: Option<&'a スキニング描画入力>,
    布入力: Option<&'a 布描画入力>,
) -> (Option<graph::バッファハンドル>, Option<scene_pass::布ドロー<'a>>) {
    let スキン済み = スキニング入力.map(|入力| {
        let ハンドル = グラフ.バッファを登録する(入力.出力バッファ, graph::前フレーム頂点入力読み直後状態());
        グラフ.パスを積む(skinning_pass::作る(ハンドル, 入力));
        ハンドル
    });
    let 布 = 布入力.map(|入力| {
        let ハンドル = cloth_passes::登録する(グラフ, 入力);
        cloth_passes::積む(グラフ, 入力, &ハンドル, スキン済み);
        scene_pass::布ドロー {
            入力,
            頂点ハンドル: ハンドル.布頂点,
        }
    });
    (スキン済み, 布)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn シーンを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &base_images::基本画像ハンドル,
    カラー: graph::画像ハンドル,
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ: &'a [ジオメトリ入力],
    シャドウ: &'a [シャドウ描画入力],
    寸法: vk::Extent2D,
) {
    グラフ.パスを積む(shadow_pass::作る(基本.シャドウマップ, スキン済み, 布, シャドウ));
    グラフ.パスを積む(scene_pass::作る(
        カラー,
        基本.深度,
        基本.シャドウマップ,
        スキン済み,
        布,
        クリア色,
        pipeline,
        ジオメトリ,
        寸法,
    ));
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 後段を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &base_images::基本画像ハンドル,
    ポスト: Option<&post_setup::ポスト構成<'a>>,
    粒子: Option<&'a 粒子描画入力>,
    ui: Option<&'a UI描画入力>,
    描画方式: &'a 描画方式,
    カラー: graph::画像ハンドル,
    寸法: vk::Extent2D,
) {
    if let Some(入力) = 粒子 {
        let ハンドル = グラフ.バッファを登録する(入力.バッファ, graph::前フレーム粒子読み直後状態());
        グラフ.パスを積む(particle_update_pass::作る(ハンドル, 入力));
        グラフ.パスを積む(particle_draw_pass::作る(カラー, 基本.深度, ハンドル, 入力, 寸法));
    }
    if let Some(構成) = ポスト {
        post_passes::積む(グラフ, 構成, 基本.スワップチェーン, 寸法);
    }
    if let Some(入力) = ui {
        グラフ.パスを積む(ui_pass::作る(基本.スワップチェーン, 入力, 寸法));
    }
    if let 描画方式::読み戻し { バッファ } = 描画方式 {
        グラフ.パスを積む(readback_pass::作る(基本.スワップチェーン, *バッファ, 寸法));
    }
}
