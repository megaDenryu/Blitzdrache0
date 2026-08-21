//! 起動時シーンの綴りの正本。担当するのは、blitz_appが名前で方針を分けるすべての綴りと、群を括る接頭辞を
//! 1箇所へ持つことである。綴りから世界の種別を解く工程は`parse`が持つ。
//!
//! 台帳を工程から分けるのは、世界が増えるたびにこの一覧だけが伸び、見分けの工程と押し合うためである。
//! アセットコンパイラ側で世界ごとの台帳を`argument_name`と`directory_source_path`へ分けているのと同じ分け方である。
//!
//! 綴りは実行時アセットの安定IDそのものであり、アセット側の宣言と同じ文字列でなければ読み込みが失敗する。

/// `--scene`の指定が無い起動が読むシーンの綴り。
pub(super) const 平面板の綴り: &str = "quad";
pub(super) const ヘルメットの綴り: &str = "helmet";
pub(super) const 影の検収世界の綴り: &str = "shadow_scene";
pub(super) const キツネの綴り: &str = "fox";
/// 材質差し替えの検収世界の綴り。上書きされる側の生成物の安定IDでもある。
pub(super) const 材質差し替えの検収世界の綴り: &str = "material_reload";
pub(super) const 間接照明の検収世界の綴り: &str = "indirect_probe";
pub(super) const 両視錐台外の群の検収世界の綴り: &str = "instance_all_culled";
pub(super) const 地形の起点世界の綴り: &str = "terrain_origin";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/terrain_visual_world.rs`
pub(super) const 地形の目視見本世界の綴り: &str = "terrain_visual";
/// 参照: `crates/blitz_asset_compiler/src/runtime_compilation/world/fox_tour_declaration.rs`
pub(super) const 地形の場所巡り世界の綴り: &str = "terrain_fox_tour";
/// 局所光の件数の上書きを読む2つの世界の一方。失敗の文面が`--scene`へ渡すべき値をそのまま出すため公開する。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/night_lights_world.rs`
pub(crate) const 地形の夜灯り世界の綴り: &str = "terrain_night_lights";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/village_world.rs`
pub(super) const 集落の小物世界の綴り: &str = "prop_village";
/// 10軒の世界と100軒の世界が同じ綴りのシーンを持つ。地面も規則も同じであり、違うのは焼いた件数だけである。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/part_house_row_world.rs`
pub(super) const 部品で組んだ家の並びの小物世界の綴り: &str = "prop_part_house_row";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/part_tree_row_world.rs`
pub(super) const 部品で組んだ木の並びの小物世界の綴り: &str = "prop_part_tree_row";
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/part_frame_row_world.rs`
pub(super) const 部品で組んだ一間四方の骨格の並びの小物世界の綴り: &str = "prop_part_frame_row";
/// 局所光の件数の上書きを読む2つの世界の一方。失敗の文面が`--scene`へ渡すべき値をそのまま出すため公開する。
/// 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/stone_hut_world.rs`
pub(crate) const 石の小屋の屋内の小物世界の綴り: &str = "prop_stone_hut_interior";

/// 床を群と同居させた植生の検収世界の綴り。どれも同じ方向光の向きを使う。
pub(super) const 床を同居させた植生の検収世界の綴り一覧: [&str; 3] = ["vegetation_cull", "vegetation_shadow_range", "vegetation_single"];

pub(super) const 植生の検収世界の接頭辞: &str = "vegetation_";
pub(super) const 小物世界の接頭辞: &str = "prop_";
pub(super) const 地形世界の接頭辞: &str = "terrain_";
pub(super) const 二材質の検収世界の接頭辞: &str = "multi_material";
pub(super) const テクスチャ圧縮の検収世界の接頭辞: &str = "texture_compression";
