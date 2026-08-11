//! 点光源の影の段階を、影を落とす灯×6面のパスへ展開する工程。受け取るのは立方体配列のハンドルと層ごとのビューと
//! そのフレームの描画計画、返す代わりにグラフへ灯の件数×6本のパスを積む。灯と面の走査順を持つのはこのファイルだけである。
//!
//! 灯を1件も持たないフレームでは1本も積まない。パスを0本積むことと、パスを積んで何も描かないことは別物である。
//! 立方体の層の内容はそのフレーム誰も標本しないためであり、消去だけのために24本を積む理由が無い。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use crate::cube_image::立方体の全面;
use crate::vulkan::frame::record::point_light_shadow_pass;
use crate::vulkan::frame::{共有セット束縛, 点光源の影の描画発行, 点光源の影の束縛};
use crate::vulkan::graph;
use crate::vulkan::point_light_shadow_plan::点光源の影の描画計画;

pub(in crate::vulkan::frame::record::graph_build) fn 点光源の影を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    層別のビュー一覧: &[vk::ImageView],
    計画: 点光源の影の描画計画,
    束縛: 点光源の影の束縛,
    発行一覧: &'a [点光源の影の描画発行],
    共有: 共有セット束縛<'a>,
) {
    for 灯 in 計画.灯を宣言の順に返す() {
        for 面 in 立方体の全面 {
            let 層番号 = 灯.影資源添字.面の層番号を求める(面);
            let 添字 = usize::try_from(層番号).unwrap_or_else(|_| panic!("点光源の影の層番号がusizeに収まらない: {層番号}"));
            let 層ビュー = *層別のビュー一覧
                .get(添字)
                .unwrap_or_else(|| panic!("点光源の影の層番号{層番号}に対応するビューが無い(層の割り当ての規約が破れている)"));
            let 描き先 = point_light_shadow_pass::点光源の影の描き先 {
                画像: 基本.点光源の影,
                層ビュー,
            };
            グラフ.パスを積む(point_light_shadow_pass::点光源の影パスを宣言する(
                灯,
                面,
                描き先,
                束縛,
                発行一覧,
                共有,
            ));
        }
    }
}
