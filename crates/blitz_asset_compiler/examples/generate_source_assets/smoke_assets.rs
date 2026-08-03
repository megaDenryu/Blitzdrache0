//! assets/smoke/ へ書き出す検証用ソースアセット一式。担当するのは、このディレクトリに何が並ぶかである。
//! quadのホットリロード検証・シャドウ検証・材質境界の検収が同じディレクトリを読むため、書き手をここへ集める。
//! 出力先ディレクトリの選択と世界ごとの並びは`main`が持ち、この工程は渡されたディレクトリへ書くことだけを行う。

use std::path::Path;

use super::{geometry, gltf_json, indirect_probe_geometry, indirect_probe_gltf_json};
use super::{multi_material_geometry, multi_material_gltf_json};
use super::{shadow_scene_geometry, shadow_scene_gltf_json, shadow_scene_texture, textures};

pub(super) fn 書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(&出力先ディレクトリ.join("quad.gltf"), gltf_json::文書JSON.as_bytes())?;
    let 代替文書 = gltf_json::文書JSON.replace("quad_base_color.png", "quad_alt_color.png");
    書き込む(&出力先ディレクトリ.join("quad_alt.gltf"), 代替文書.as_bytes())?;
    書き込む(&出力先ディレクトリ.join("quad.bin"), &geometry::バッファバイト列を作る())?;
    textures::保存する(出力先ディレクトリ)?;
    材質境界アセットを書き出す(出力先ディレクトリ)?;
    遠方環境の検収アセットを書き出す(出力先ディレクトリ)?;
    シャドウ検証アセットを書き出す(出力先ディレクトリ)
}

/// 材質境界の検収アセット。同じ形と同じバッファを、2材質2プリミティブと1材質1プリミティブの2つの文書が読む。
/// 材質の係数だけを差し替えた3つ目の文書も同じバッファを読む。差し替えの前後で形が変わらないことが、
/// 画素の違いを材質の違いだけに帰属させる根拠になる。
fn 材質境界アセットを書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(
        &出力先ディレクトリ.join("multi_material.bin"),
        &multi_material_geometry::バッファバイト列を作る(),
    )?;
    書き込む(
        &出力先ディレクトリ.join("multi_material_two.gltf"),
        multi_material_gltf_json::二材質の文書().as_bytes(),
    )?;
    書き込む(
        &出力先ディレクトリ.join("multi_material_two_alt.gltf"),
        multi_material_gltf_json::代替の二材質の文書().as_bytes(),
    )?;
    書き込む(
        &出力先ディレクトリ.join("multi_material_one.gltf"),
        multi_material_gltf_json::単一材質の文書().as_bytes(),
    )
}

/// 遠方環境の消費の検収アセット。金属と誘電体の板を粗さの水準ごとに並べた1つの世界であり、
/// 板ごとに材質とプリミティブが分かれる。
fn 遠方環境の検収アセットを書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    書き込む(
        &出力先ディレクトリ.join("indirect_probe.bin"),
        &indirect_probe_geometry::バッファバイト列を作る(),
    )?;
    書き込む(
        &出力先ディレクトリ.join("indirect_probe.gltf"),
        indirect_probe_gltf_json::文書().as_bytes(),
    )
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
