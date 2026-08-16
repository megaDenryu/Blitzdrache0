//! 起動時シーンの綴りの正本と、綴りから世界の種別を見分ける工程。担当するのは、blitz_appが名前で方針を
//! 分けるすべての綴りを1箇所へ持ち、`--scene`に来た1つの綴りをどの世界かへ解くことである。
//!
//! 綴りは実行時アセットの安定IDそのものであり、アセット側の宣言と同じ文字列でなければ読み込みが失敗する。
//! 名前ごとの世界を先に見分け、当たらなかった綴りだけを接頭辞の群へ回すのは、群の中の名前つきの世界
//! (`terrain_visual`等)が群の既定より先に自分の方針を選ぶためである。

use super::world_kind::{世界の種別, 地形世界の種別, 小物世界の種別, 植生の検収世界の種別};

/// `--scene`の指定が無い起動が読むシーンの綴り。
pub(super) const 平面板の綴り: &str = "quad";
const ヘルメットの綴り: &str = "helmet";
const 影の検収世界の綴り: &str = "shadow_scene";
const キツネの綴り: &str = "fox";
/// 材質差し替えの検収世界の綴り。上書きされる側の生成物の安定IDでもある。
pub(super) const 材質差し替えの検収世界の綴り: &str = "material_reload";
const 間接照明の検収世界の綴り: &str = "indirect_probe";
const 両視錐台外の群の検収世界の綴り: &str = "instance_all_culled";
const 地形の起点世界の綴り: &str = "terrain_origin";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/terrain_visual_world.rs`
const 地形の目視見本世界の綴り: &str = "terrain_visual";
/// 参照: `crates/blitz_asset_compiler/examples/compile_assets/world/fox_tour_declaration.rs`
const 地形の場所巡り世界の綴り: &str = "terrain_fox_tour";
/// 局所光の件数の上書きを読む2つの世界の一方。失敗の文面が`--scene`へ渡すべき値をそのまま出すため公開する。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/night_lights_world.rs`
pub(crate) const 地形の夜灯り世界の綴り: &str = "terrain_night_lights";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/village_world.rs`
const 集落の小物世界の綴り: &str = "prop_village";
/// 10軒の世界と100軒の世界が同じ綴りのシーンを持つ。地面も規則も同じであり、違うのは焼いた件数だけである。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/part_house_row_world.rs`
const 部品で組んだ家の並びの小物世界の綴り: &str = "prop_part_house_row";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/part_tree_row_world.rs`
const 部品で組んだ木の並びの小物世界の綴り: &str = "prop_part_tree_row";
/// 局所光の件数の上書きを読む2つの世界の一方。失敗の文面が`--scene`へ渡すべき値をそのまま出すため公開する。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/stone_hut_world.rs`
pub(crate) const 石の小屋の屋内の小物世界の綴り: &str = "prop_stone_hut_interior";

/// 床を群と同居させた植生の検収世界の綴り。どれも同じ方向光の向きを使う。
const 床を同居させた植生の検収世界の綴り一覧: [&str; 3] = ["vegetation_cull", "vegetation_shadow_range", "vegetation_single"];

const 植生の検収世界の接頭辞: &str = "vegetation_";
const 小物世界の接頭辞: &str = "prop_";
const 地形世界の接頭辞: &str = "terrain_";
const 二材質の検収世界の接頭辞: &str = "multi_material";
const テクスチャ圧縮の検収世界の接頭辞: &str = "texture_compression";

pub(super) fn 綴りから世界の種別を見分ける(綴り: &str) -> 世界の種別 {
    if let Some(種別) = 名前つきの世界を見分ける(綴り) {
        return 種別;
    }
    接頭辞の群を見分ける(綴り)
}

fn 名前つきの世界を見分ける(綴り: &str) -> Option<世界の種別> {
    let 種別 = match 綴り {
        平面板の綴り => 世界の種別::平面板,
        ヘルメットの綴り => 世界の種別::ヘルメット,
        影の検収世界の綴り => 世界の種別::影の検収世界,
        キツネの綴り => 世界の種別::キツネ,
        材質差し替えの検収世界の綴り => 世界の種別::材質差し替えの検収世界,
        間接照明の検収世界の綴り => 世界の種別::間接照明の検収世界,
        両視錐台外の群の検収世界の綴り => 世界の種別::両視錐台外の群の検収世界,
        地形の起点世界の綴り => 世界の種別::地形世界(地形世界の種別::起点),
        地形の目視見本世界の綴り => 世界の種別::地形世界(地形世界の種別::目視見本),
        地形の場所巡り世界の綴り => 世界の種別::地形世界(地形世界の種別::場所巡り),
        地形の夜灯り世界の綴り => 世界の種別::地形世界(地形世界の種別::夜灯り),
        集落の小物世界の綴り => 世界の種別::小物世界(小物世界の種別::集落),
        部品で組んだ家の並びの小物世界の綴り => 世界の種別::小物世界(小物世界の種別::部品で組んだ家の並び),
        部品で組んだ木の並びの小物世界の綴り => 世界の種別::小物世界(小物世界の種別::部品で組んだ木の並び),
        石の小屋の屋内の小物世界の綴り => 世界の種別::小物世界(小物世界の種別::石の小屋の屋内),
        名前つきでない綴り => {
            if 床を同居させた植生の検収世界の綴り一覧.contains(&名前つきでない綴り) {
                世界の種別::植生の検収世界(植生の検収世界の種別::床を同居させた)
            } else {
                return None;
            }
        }
    };
    Some(種別)
}

fn 接頭辞の群を見分ける(綴り: &str) -> 世界の種別 {
    if 綴り.starts_with(植生の検収世界の接頭辞) {
        世界の種別::植生の検収世界(植生の検収世界の種別::床を持たない)
    } else if 綴り.starts_with(小物世界の接頭辞) {
        世界の種別::小物世界(小物世界の種別::名前で選ばれない小物)
    } else if 綴り.starts_with(地形世界の接頭辞) {
        世界の種別::地形世界(地形世界の種別::名前で選ばれない地形)
    } else if 綴り.starts_with(二材質の検収世界の接頭辞) {
        世界の種別::二材質の検収世界
    } else if 綴り.starts_with(テクスチャ圧縮の検収世界の接頭辞) {
        世界の種別::テクスチャ圧縮の検収世界
    } else {
        世界の種別::名前で選ばれない世界
    }
}
