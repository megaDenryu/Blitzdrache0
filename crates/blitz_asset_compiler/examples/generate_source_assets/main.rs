//! ソースアセットの生成器。決定的・ネット不要。書き出すものは起動引数で2つに分かれる。
//!
//! 引数なし(`cargo xtask gen-source-assets`)では検証用の世界一式を書き出す。行き先は
//! assets/smoke/ へ quad.gltf・quad_alt.gltf・quad.bin・2色のテクスチャと、
//! shadow_scene.gltf・shadow_scene.bin・shadow_scene_white.png(判断37)と、
//! 材質境界の検収用のmulti_material.bin・multi_material_two.gltf・multi_material_one.gltfと、
//! 遠方環境の消費の検収用のindirect_probe.bin・indirect_probe.gltfを、
//! assets/chunk_world/ へ25チャンク分のglTFと共有バッファと目録ソースを、
//! assets/terrain_world/ へ25チャンク分の高さ格子と目録ソースを、
//! assets/vegetation_world/ へ植生の原型glTFと頂点量の診断用に面を細分化した原型glTFと1チャンクの目録ソースを、
//! assets/village_world/ へ見本の集落の地面1チャンク分の高さ格子と目録ソースを、
//! assets/part_house_row_world/ へ部品で組んだ家の並びの平らな地面1チャンク分の高さ格子と目録ソースを、
//! assets/part_tree_row_world/ へ部品で組んだ木の並びの平らな地面1チャンク分の高さ格子と目録ソースを、
//! assets/terrain_visual_world/ へ目視見本の地面1チャンク分の高さ格子と目録ソースと材質見本の立体のglTFを、
//! assets/night_lights_world/ へ夜の多光源の地面1チャンク分の高さ格子と目録ソースを、
//! assets/stone_hut_world/ へ屋内の多光源の平らな地面1チャンク分の高さ格子と目録ソースを、
//! assets/texture_compression_world/ へブロック圧縮の対照の素材(512画素四方の滑らかなグラデーションと決定的な雑音)と
//! それらをベースカラーに持つ板2枚のglTFと1チャンクの目録ソースを書き出す。
//!
//! `--game-map-seed <数>`(`cargo xtask gen-game-map --seed <数>`)では場所巡りの世界だけを書き出す。行き先は
//! ソースルート(既定はassets/、`--source-root <パス>`で差し替える)の下のfox_tour_world/であり、種から決めた
//! 9チャンク分の高さ格子と目録ソースと目的地の目印の柱のglTFと、生成に使った種そのものを書いたファイルと、
//! 焼き直しを省くための生成台帳である。
//!
//! どちらの実行もリポジトリルートを作業ディレクトリとする。

mod chunk_world;
mod directory_source;
mod fox_tour_world;
mod generation_arguments;
mod geometry;
mod gltf_json;
mod indirect_probe_geometry;
mod indirect_probe_gltf_json;
mod indirect_probe_plates;
mod multi_material_geometry;
mod multi_material_gltf_json;
mod night_lights_world;
mod part_frame_row_world;
mod part_house_row_world;
mod part_tree_row_world;
mod shadow_scene_geometry;
mod shadow_scene_gltf_json;
mod shadow_scene_texture;
mod smoke_assets;
mod stone_hut_world;
mod terrain_visual_world;
mod terrain_world;
mod texture_compression_world;
mod textures;
mod vegetation_world;
mod verification_worlds;
mod village_world;

use std::path::Path;

use generation_arguments::書き出す対象;

fn main() {
    if let Err(誤り) = 実行する() {
        eprintln!("[generate_source_assets] {誤り}");
        std::process::exit(1);
    }
}

fn 実行する() -> Result<(), String> {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match generation_arguments::引数一覧から書き出す対象を読む(&引数一覧)? {
        書き出す対象::場所巡りの世界 {
            種, ソースルート, 広がり
        } => fox_tour_world::書き出す(&ソースルート, 種, 広がり).map(|_| ()),
        書き出す対象::検証用の世界一式 => verification_worlds::一式を書き出す(),
    }
}

pub(crate) fn ディレクトリを作る(パス: &Path) -> Result<(), String> {
    std::fs::create_dir_all(パス).map_err(|誤り| format!("{}の作成に失敗した: {誤り}", パス.display()))
}
