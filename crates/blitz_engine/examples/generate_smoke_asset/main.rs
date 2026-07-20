//! スモーク用極小自作glTFアセットの生成器。決定的・ネット不要。
//! `cargo run -p blitz_engine --example generate_smoke_asset` で
//! assets/smoke/ へ quad.gltf・quad.bin・quad_base_color.png・quad_alt_color.png と、
//! shadow_scene.gltf・shadow_scene.bin・shadow_scene_white.png(判断37)を書き出す。
//! xtask gen-smoke-asset の実体であり、リポジトリルートを作業ディレクトリとして実行される。

mod geometry;
mod gltf_json;
mod shadow_scene_geometry;
mod shadow_scene_gltf_json;
mod shadow_scene_texture;
mod textures;

use std::path::Path;

fn main() {
    let 出力先ディレクトリ = Path::new("assets/smoke");
    if let Err(誤り) = std::fs::create_dir_all(出力先ディレクトリ) {
        eprintln!("[generate_smoke_asset] 出力先ディレクトリの作成に失敗した: {誤り}");
        std::process::exit(1);
    }

    if let Err(誤り) = std::fs::write(出力先ディレクトリ.join("quad.gltf"), gltf_json::文書JSON) {
        eprintln!("[generate_smoke_asset] quad.gltfの書き出しに失敗した: {誤り}");
        std::process::exit(1);
    }

    let バイト列 = geometry::バッファバイト列を作る();
    if let Err(誤り) = std::fs::write(出力先ディレクトリ.join("quad.bin"), バイト列) {
        eprintln!("[generate_smoke_asset] quad.binの書き出しに失敗した: {誤り}");
        std::process::exit(1);
    }

    if let Err(誤り) = textures::保存する(出力先ディレクトリ) {
        eprintln!("[generate_smoke_asset] テクスチャの書き出しに失敗した: {誤り}");
        std::process::exit(1);
    }

    if let Err(誤り) = シャドウ検証アセットを書き出す(出力先ディレクトリ) {
        eprintln!("[generate_smoke_asset] シャドウ検証アセットの書き出しに失敗した: {誤り}");
        std::process::exit(1);
    }

    println!("[generate_smoke_asset] {}へ生成完了", 出力先ディレクトリ.display());
}

fn シャドウ検証アセットを書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    std::fs::write(出力先ディレクトリ.join("shadow_scene.gltf"), shadow_scene_gltf_json::文書JSON)
        .map_err(|誤り| format!("shadow_scene.gltf: {誤り}"))?;
    std::fs::write(
        出力先ディレクトリ.join("shadow_scene.bin"),
        shadow_scene_geometry::バッファバイト列を作る(),
    )
    .map_err(|誤り| format!("shadow_scene.bin: {誤り}"))?;
    shadow_scene_texture::保存する(出力先ディレクトリ)
}
