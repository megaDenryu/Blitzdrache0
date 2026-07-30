//! 影段階を帯ごとのシャドウパスへ展開する工程。受け取るのは帯ごとのアタッチメントビューと帯別のシャドウ候補、
//! 返す代わりにグラフへ帯数ぶんのパスを積む。帯の走査順を持つのはこのファイルだけである。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use crate::cascade::{帯数, 帯番号};
use crate::vulkan::frame::record::{scene_pass, shadow_pass};
use crate::vulkan::frame::帯別シャドウ入力;
use crate::vulkan::graph;

/// 帯ごとに1回ずつパスを積み、その帯のライト視錐台が通した候補だけを渡す(第6段の帯別カリング)。
pub(in crate::vulkan::frame::record::graph_build) fn 影を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    帯ビュー一覧: &[vk::ImageView; 帯数],
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    入力: 帯別シャドウ入力<'a>,
) {
    for 番号 in 帯番号::全帯() {
        グラフ.パスを積む(shadow_pass::作る(
            番号,
            基本.シャドウマップ,
            帯ビュー一覧[番号.添字()],
            スキン済み,
            布,
            入力.帯(番号),
        ));
    }
}
