//! 検証用ソースアセットの生成器。決定的・ネット不要。
//! `cargo xtask gen-source-assets`で
//! assets/smoke/ へ quad.gltf・quad_alt.gltf・quad.bin・2色のテクスチャと、
//! shadow_scene.gltf・shadow_scene.bin・shadow_scene_white.png(判断37)を、
//! assets/chunk_world/ へ25チャンク分のglTFと共有バッファと目録ソースを、
//! assets/terrain_world/ へ25チャンク分の高さ格子と目録ソースを、
//! assets/vegetation_world/ へ植生の原型glTFと1チャンクの目録ソースを書き出す。
//! xtask gen-source-assets の実体であり、リポジトリルートを作業ディレクトリとして実行される。

mod chunk_world;
mod directory_source;
mod geometry;
mod gltf_json;
mod shadow_scene_geometry;
mod shadow_scene_gltf_json;
mod shadow_scene_texture;
mod terrain_world;
mod textures;
mod vegetation_world;

use std::path::Path;

fn main() {
    if let Err(誤り) = 実行する() {
        eprintln!("[generate_source_assets] {誤り}");
        std::process::exit(1);
    }
}

fn 実行する() -> Result<(), String> {
    let スモーク出力先 = Path::new("assets/smoke");
    ディレクトリを作る(スモーク出力先)?;
    スモークアセットを書き出す(スモーク出力先)?;
    println!("[generate_source_assets] {}へ生成完了", スモーク出力先.display());

    let チャンク世界出力先 = Path::new("assets/chunk_world");
    ディレクトリを作る(チャンク世界出力先)?;
    chunk_world::書き出す(チャンク世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", チャンク世界出力先.display());

    let 地形世界出力先 = Path::new("assets/terrain_world");
    ディレクトリを作る(地形世界出力先)?;
    terrain_world::書き出す(地形世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 地形世界出力先.display());

    let 植生世界出力先 = Path::new("assets/vegetation_world");
    ディレクトリを作る(植生世界出力先)?;
    vegetation_world::書き出す(植生世界出力先)?;
    println!("[generate_source_assets] {}へ生成完了", 植生世界出力先.display());
    Ok(())
}

fn ディレクトリを作る(パス: &Path) -> Result<(), String> {
    std::fs::create_dir_all(パス).map_err(|誤り| format!("{}の作成に失敗した: {誤り}", パス.display()))
}

fn スモークアセットを書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(&出力先ディレクトリ.join("quad.gltf"), gltf_json::文書JSON.as_bytes())?;
    let 代替文書 = gltf_json::文書JSON.replace("quad_base_color.png", "quad_alt_color.png");
    書き込む(&出力先ディレクトリ.join("quad_alt.gltf"), 代替文書.as_bytes())?;
    書き込む(&出力先ディレクトリ.join("quad.bin"), &geometry::バッファバイト列を作る())?;
    textures::保存する(出力先ディレクトリ)?;
    シャドウ検証アセットを書き出す(出力先ディレクトリ)
}

fn シャドウ検証アセットを書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(&出力先ディレクトリ.join("shadow_scene.gltf"), shadow_scene_gltf_json::文書JSON.as_bytes())?;
    書き込む(
        &出力先ディレクトリ.join("shadow_scene.bin"),
        &shadow_scene_geometry::バッファバイト列を作る(),
    )?;
    shadow_scene_texture::保存する(出力先ディレクトリ)
}

fn 書き込む(パス: &Path, バイト列: &[u8]) -> Result<(), String> {
    std::fs::write(パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
