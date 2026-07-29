//! 影段階を帯ごとのシャドウパスへ展開する工程。受け取るのは帯ごとのアタッチメントビューとシャドウ候補一覧、
//! 返す代わりにグラフへ帯数ぶんのパスを積む。帯の走査順を持つのはこのファイルだけである。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use crate::cascade::{帯数, 帯番号};
use crate::vulkan::frame::record::{scene_pass, shadow_pass};
use crate::vulkan::frame::シャドウ描画入力;
use crate::vulkan::graph;

/// 帯ごとに1回ずつパスを積む。この段では帯別のカリングを持たないため、どの帯にも同じ候補一覧を渡す。
pub(in crate::vulkan::frame::record::graph_build) fn 影を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    帯ビュー一覧: &[vk::ImageView; 帯数],
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    入力: &'a [シャドウ描画入力],
) {
    for 番号 in 帯番号::全帯() {
        グラフ.パスを積む(shadow_pass::作る(
            番号,
            基本.シャドウマップ,
            帯ビュー一覧[番号.添字()],
            スキン済み,
            布,
            入力,
        ));
    }
}
