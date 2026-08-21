//! 複数の世界または開発用入口が共有するソースアセットの相対パス。
//! 同じファイルの綴りを宣言と複製処理が別々に持たないため、この一覧を正本にする。

pub const フォックスのソース: &str = "samples/Fox/Fox.glb";
pub const ヘルメットのソース: &str = "samples/DamagedHelmet/DamagedHelmet.glb";
pub const 植生診断原型のソース: &str = "vegetation_world/archetype_lod.gltf";
pub const 目印の柱のファイル名: &str = "destination_marker.gltf";
pub const 目印の柱のソース: &str = concat!("fox_tour_world/", "destination_marker.gltf");
pub const 板のファイル名: &str = "quad.gltf";
pub const 代替板のファイル名: &str = "quad_alt.gltf";
pub const 影のシーンのファイル名: &str = "shadow_scene.gltf";
pub const 二材質のファイル名: &str = "multi_material_two.gltf";
pub const 代替二材質のファイル名: &str = "multi_material_two_alt.gltf";
pub const 単一材質のファイル名: &str = "multi_material_one.gltf";
pub const 遠方環境の検収ファイル名: &str = "indirect_probe.gltf";
pub const 板のソース: &str = concat!("smoke/", "quad.gltf");
pub const 代替板のソース: &str = concat!("smoke/", "quad_alt.gltf");
pub const 影のシーンのソース: &str = concat!("smoke/", "shadow_scene.gltf");
pub const 二材質のソース: &str = concat!("smoke/", "multi_material_two.gltf");
pub const 代替二材質のソース: &str = concat!("smoke/", "multi_material_two_alt.gltf");
pub const 単一材質のソース: &str = concat!("smoke/", "multi_material_one.gltf");
pub const 遠方環境の検収ソース: &str = concat!("smoke/", "indirect_probe.gltf");
pub const 小物の柵のソース: &str = "props/fence_section.glb";
pub const 小物の樽のソース: &str = "props/barrel.glb";
pub const 小物の木箱のソース: &str = "props/wooden_crate.glb";
pub const 小物の岩のソース: &str = "props/boulder.glb";
pub const 小物の切り株のソース: &str = "props/tree_stump.glb";
pub const 小物の小石のソース: &str = "props/rock.glb";
pub const 針葉樹のソース: &str = "props/conifer.glb";
