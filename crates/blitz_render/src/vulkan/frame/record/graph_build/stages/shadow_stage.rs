//! 影段階を距離区分ごとのシャドウパスへ展開する工程。受け取るのは距離区分ごとのアタッチメントビューと距離区分別のシャドウ候補、
//! 返す代わりにグラフへ距離区分数ぶんのパスを積む。距離区分の走査順を持つのはこのファイルだけである。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use crate::cascade::{距離区分数, 距離区分番号};
use crate::vulkan::frame::record::{scene_pass, shadow_pass};
use crate::vulkan::frame::{共有セット束縛, 距離区分別のシャドウ入力};
use crate::vulkan::graph;

/// 距離区分ごとに1回ずつパスを積み、その距離区分のライト視錐台が通した候補だけを渡す(第6段の距離区分別の可視個体の選別)。
pub(in crate::vulkan::frame::record::graph_build) fn 影を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    距離区分別のビュー一覧: &[vk::ImageView; 距離区分数],
    スキン済み: Option<graph::バッファハンドル>,
    布: Option<scene_pass::布ドロー<'a>>,
    入力: 距離区分別のシャドウ入力<'a>,
    共有: 共有セット束縛<'a>,
) {
    for 番号 in 距離区分番号::全距離区分() {
        let 描き先 = shadow_pass::シャドウの描き先 {
            画像: 基本.シャドウマップ,
            層ビュー: 距離区分別のビュー一覧[番号.添字()],
            一辺: 基本.シャドウマップ一辺,
        };
        グラフ.パスを積む(shadow_pass::シャドウ描画パスを宣言する(
            番号,
            描き先,
            スキン済み,
            布,
            入力.距離区分(番号),
            共有,
        ));
    }
}
