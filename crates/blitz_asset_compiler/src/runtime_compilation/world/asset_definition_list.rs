//! 世界ごとに、その世界がチャンク以外へ焼くアセットの一覧を引き当てる台帳。中身は世界ごとの宣言モジュールが持ち、
//! ここが持つのは「どの世界がどの宣言を読むか」の対応だけである。
//!
//! 台帳を親から分けるのは、世界が増えるたびにこの一覧だけが伸び、対象世界の枝の定義と押し合うためである。
//! チャンクのソース形式の台帳を`chunk_source_kind`が、目録ソースの置き場を`directory_source_path`が持つのと
//! 同じ分け方である。

use super::super::catalog::アセット定義;
use super::{
    asset_declaration, fox_declaration, fox_tour_declaration, night_lights_declaration, part_frame_row_declaration, part_house_row_declaration,
    part_tree_row_declaration, stone_hut_declaration, vegetation_declaration, vertex_diagnostic_declaration, village_declaration,
    visual_sample_declaration, 対象世界,
};

pub(super) fn アセット定義一覧を選ぶ(世界: 対象世界) -> Vec<アセット定義> {
    match 世界 {
        対象世界::板の世界 => asset_declaration::板の世界の一覧(),
        対象世界::地形の世界 => asset_declaration::地形の世界の一覧(),
        対象世界::エディターの世界 => {
            let mut 一覧 = vec![fox_declaration::キツネのシーン定義を作る("terrain_editor_world")];
            let 素材の識別一覧 = crate::runtime_compilation::building_outline_catalog::全建物の部品識別一覧()
                .into_iter()
                .chain(crate::runtime_compilation::vegetation_definition::全植生の原型識別一覧());
            一覧.extend(素材の識別一覧.map(|(安定id, 相対パス)| super::definition_kind::外部ソース専用定義(安定id, 相対パス)));
            一覧
        }
        対象世界::植生の世界 => vegetation_declaration::一覧(),
        対象世界::見本の集落の世界 => village_declaration::一覧(),
        対象世界::部品で組んだ家の並びの世界(_) => part_house_row_declaration::一覧(),
        対象世界::部品で組んだ木の並びの世界 => part_tree_row_declaration::一覧(),
        対象世界::部品で組んだ一間四方の骨格の並びの世界(種類) => part_frame_row_declaration::一覧(種類),
        対象世界::目視見本の世界 => visual_sample_declaration::一覧(),
        対象世界::頂点診断の世界(原型) => vertex_diagnostic_declaration::一覧(原型),
        対象世界::ブロック圧縮の対照世界 => Vec::new(),
        対象世界::夜の多光源の世界 => night_lights_declaration::一覧(),
        対象世界::屋内の多光源の世界 => stone_hut_declaration::一覧(),
        対象世界::場所巡りの世界 => fox_tour_declaration::一覧(),
    }
}
