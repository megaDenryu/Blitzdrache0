//! 高水準なフレーム段階をレンダーグラフの具体的なパスへ展開する。

use ash::vk;

mod atmosphere_lut_stage;
mod shadow_stage;
mod sky_stage;

pub(super) use atmosphere_lut_stage::大気のベイク済み画像を積む;
pub(super) use shadow_stage::影を積む;
pub(super) use sky_stage::{空を積む, 空パスがベイク済み画像を参照するか};

use super::base_images::基本画像ハンドル;
use crate::clear_color::クリアカラー;
use crate::vulkan::frame::record::{cloth_passes, particle_draw_pass, particle_update_pass, scene_pass, skinning_pass};
use crate::vulkan::frame::{ジオメトリ入力, スキニング描画入力, 共有セット束縛, 布描画入力, 粒子描画入力};
use crate::vulkan::graph;

pub(super) fn スキニングを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    入力: Option<&'a スキニング描画入力>,
) -> Option<graph::バッファハンドル> {
    入力.map(|入力| {
        let ハンドル = グラフ.バッファを登録する(入力.出力バッファ, graph::前フレーム頂点入力読み直後状態());
        グラフ.パスを積む(skinning_pass::作る(ハンドル, 入力));
        ハンドル
    })
}

pub(super) fn 布を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    入力: Option<&'a 布描画入力>,
    スキン済み: Option<graph::バッファハンドル>,
) -> Option<scene_pass::布ドロー<'a>> {
    入力.map(|入力| {
        let ハンドル = cloth_passes::登録する(グラフ, 入力);
        cloth_passes::積む(グラフ, 入力, &ハンドル, スキン済み);
        scene_pass::布ドロー {
            入力,
            頂点ハンドル: ハンドル.布頂点,
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn シーンを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    カラー: graph::画像ハンドル,
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    入力: &'a [ジオメトリ入力],
    共有: 共有セット束縛,
    寸法: vk::Extent2D,
) {
    グラフ.パスを積む(scene_pass::作る(
        カラー,
        基本.深度,
        基本.シャドウマップ,
        スキン済み,
        布,
        クリア色,
        pipeline,
        入力,
        共有,
        寸法,
    ));
}

pub(super) fn 粒子を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    カラー: graph::画像ハンドル,
    入力: Option<&'a 粒子描画入力>,
    寸法: vk::Extent2D,
) {
    if let Some(入力) = 入力 {
        let ハンドル = グラフ.バッファを登録する(入力.バッファ, graph::前フレーム粒子読み直後状態());
        グラフ.パスを積む(particle_update_pass::作る(ハンドル, 入力));
        グラフ.パスを積む(particle_draw_pass::作る(カラー, 基本.深度, ハンドル, 入力, 寸法));
    }
}
