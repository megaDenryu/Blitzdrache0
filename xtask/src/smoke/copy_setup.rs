//! smoke実行前の一時コピー: shaders/配下の全.slangファイルとassets/smoke/一式を
//! target/配下へ複製する。スモークシナリオが監視対象ファイルを書き換えるため、
//! リポジトリ本体は汚さない。shaders/はimportでモジュール分割されているため
//! エントリファイル(scene.slang)単体でなくディレクトリ単位でコピーする
//! (分割先のpbr.slangがコピー先に無いとslangcのimport解決が失敗する)。

use std::path::{Path, PathBuf};

const エントリファイル名: &str = "scene.slang";
const チャンク世界ディレクトリ名: &str = "chunk_world";
/// 一時ソースへ複製するassets/smoke/のファイル。スモークが描くのはquadだけだが、実行時アセット生成器は板の世界の必須アセットが
/// 1つでも欠けると失敗するため、同じディレクトリの宣言に並ぶものはすべて複製する。
const アセットファイル一覧: [&str; 14] = [
    "quad.gltf",
    "quad_alt.gltf",
    "quad.bin",
    "quad_base_color.png",
    "quad_alt_color.png",
    "shadow_scene.gltf",
    "shadow_scene.bin",
    "shadow_scene_white.png",
    "multi_material.bin",
    "multi_material_two.gltf",
    "multi_material_two_alt.gltf",
    "multi_material_one.gltf",
    "indirect_probe.bin",
    "indirect_probe.gltf",
];

pub(super) fn シェーダーを一時コピーする() -> Result<PathBuf, String> {
    let コピー先ディレクトリ = PathBuf::from("target/smoke_shaders");
    std::fs::create_dir_all(&コピー先ディレクトリ).map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    let 読み取り結果 = std::fs::read_dir("shaders").map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果.map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
        let 元パス = エントリ.path();
        let 拡張子が対象か = 元パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("slang");
        if !元パス.is_file() || !拡張子が対象か {
            continue;
        }
        let ファイル名 = エントリ.file_name();
        std::fs::copy(&元パス, コピー先ディレクトリ.join(&ファイル名)).map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元パス.display()))?;
    }

    Ok(コピー先ディレクトリ.join(エントリファイル名))
}

pub(super) fn アセットを一時コピーする() -> Result<PathBuf, String> {
    let ルート = PathBuf::from("target/smoke_assets");
    let コピー先ディレクトリ = ルート.join("smoke");
    std::fs::create_dir_all(&コピー先ディレクトリ).map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    for ファイル名 in アセットファイル一覧 {
        let 元 = PathBuf::from("assets/smoke").join(ファイル名);
        let 先 = コピー先ディレクトリ.join(ファイル名);
        std::fs::copy(&元, &先).map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元.display()))?;
    }
    チャンク世界を複製する(&ルート)?;
    Ok(ルート)
}

/// スモークはチャンク世界を描かないが、実行時アセット生成器はチャンク目録ソースの不在を失敗として扱うため、一式を一時ソースへ複製する。
fn チャンク世界を複製する(ルート: &Path) -> Result<(), String> {
    let 元ディレクトリ = PathBuf::from("assets").join(チャンク世界ディレクトリ名);
    let 先ディレクトリ = ルート.join(チャンク世界ディレクトリ名);
    std::fs::create_dir_all(&先ディレクトリ).map_err(|誤り| format!("チャンク世界のコピー先を作れない: {誤り}"))?;
    let 読み取り結果 = std::fs::read_dir(&元ディレクトリ).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", 元ディレクトリ.display()))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果.map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", 元ディレクトリ.display()))?;
        let 元パス = エントリ.path();
        if !元パス.is_file() {
            continue;
        }
        std::fs::copy(&元パス, 先ディレクトリ.join(エントリ.file_name()))
            .map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元パス.display()))?;
    }
    Ok(())
}
