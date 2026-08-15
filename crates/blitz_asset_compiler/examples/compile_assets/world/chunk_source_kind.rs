//! 世界ごとに、そのチャンクがどのソース形式で書かれているかの台帳と引き当て。板はglTF、地形は高さ格子、植生は原型glTFである。
//! 台帳を親から分けるのは、世界が増えるたびにこの一覧だけが伸び、アセット定義の引き当てと押し合うためである。
//! 目録ソースの置き場の台帳を`directory_source_path`が持つのと同じ分け方である。
//!
//! 地形のチャンクだけが同居植生の宣言を伴う。本番のストリーミング経路で植生が出入りすることをこの世界で検査するためである。
//! 同居植生の個体数だけを外から受け取るのは、物量計測が原型・マテリアル・座標を固定したまま密度を変えるためである。

use blitz_asset_compiler::散布の焼き方;

use super::super::catalog::ソース種別;
use super::{
    asset_declaration, fox_tour_declaration, night_lights_declaration, stone_hut_declaration, vegetation_declaration, vertex_diagnostic_declaration,
    village_declaration, visual_sample_declaration, 対象世界,
};

pub(super) fn チャンクのソース種別を選ぶ(
    世界: 対象世界, 同居植生個体数: usize, 散布: 散布の焼き方
) -> ソース種別 {
    match 世界 {
        対象世界::板の世界 | 対象世界::ブロック圧縮の対照世界 => ソース種別::Gltfシーン,
        対象世界::地形の世界 => ソース種別::高さ格子 {
            同居植生: Some(asset_declaration::地形の同居植生(同居植生個体数)),
        },
        対象世界::植生の世界 => vegetation_declaration::植生種別(vegetation_declaration::計数判定の個体数),
        対象世界::見本の集落の世界 => ソース種別::見本の集落 {
            群一覧: village_declaration::集落の小物一覧,
        },
        対象世界::目視見本の世界 => ソース種別::目視見本 {
            材質見本の立体の安定id: visual_sample_declaration::材質見本の立体,
            遠景地面の安定id: visual_sample_declaration::遠景地面,
            群一覧: visual_sample_declaration::庭の小物一覧,
        },
        対象世界::頂点診断の世界(原型) => ソース種別::高さ格子 {
            同居植生: Some(vertex_diagnostic_declaration::同居植生(原型, 同居植生個体数)),
        },
        対象世界::夜の多光源の世界 => night_lights_declaration::チャンクのソース種別(),
        対象世界::屋内の多光源の世界 => stone_hut_declaration::チャンクのソース種別(),
        対象世界::場所巡りの世界 => fox_tour_declaration::チャンクのソース種別(散布),
    }
}
