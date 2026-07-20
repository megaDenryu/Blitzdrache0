//! 依存白リスト検査: 各クレートのCargo.tomlが許可された依存だけを持つか確認する。
//! 依存追加は親の採用審査 + この表の更新が必要（意図的な二重台帳。Cargo.toml側の
//! 変更だけでは通らないことが検査の目的）。

use std::path::{Path, PathBuf};

use super::cargo_toml_parse::{クレート名を取り出す, 依存名一覧を取り出す};
use super::violation::違反;

const 白リスト: [(&str, &[&str]); 5] = [
    ("blitz_math", &["glam"]),
    ("blitz_engine", &["blitz_math", "blitz_render", "gltf", "image", "thiserror"]),
    ("blitz_render", &["ash", "ash-window", "raw-window-handle", "glam", "thiserror", "blitz_math"]),
    (
        "blitz_app",
        &["blitz_engine", "blitz_math", "blitz_render", "winit", "raw-window-handle", "thiserror", "egui", "egui-winit"],
    ),
    ("xtask", &[]),
];

pub fn 全クレートを検査する() -> Result<Vec<違反>, String> {
    let mut 対象一覧: Vec<PathBuf> = Vec::new();
    let 読み取り結果 =
        std::fs::read_dir("crates").map_err(|誤り| format!("crates/の読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果.map_err(|誤り| format!("crates/の読み取りに失敗した: {誤り}"))?;
        let 候補 = エントリ.path().join("Cargo.toml");
        if 候補.is_file() {
            対象一覧.push(候補);
        }
    }
    対象一覧.push(PathBuf::from("xtask/Cargo.toml"));

    let mut 違反一覧 = Vec::new();
    for パス in &対象一覧 {
        let 内容 = std::fs::read_to_string(パス)
            .map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        違反一覧.extend(単一クレートを検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn 単一クレートを検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let クレート名 = match クレート名を取り出す(内容) {
        Some(名前) => 名前,
        None => return vec![違反::ファイル単位(パス.to_path_buf(), "パッケージ名が読み取れない".to_string())],
    };
    let 依存名一覧 = 依存名一覧を取り出す(内容);
    match 白リスト.iter().find(|(名前, _)| *名前 == クレート名) {
        Some((_, 許可一覧)) => 依存名一覧
            .into_iter()
            .filter(|依存名| !許可一覧.contains(&依存名.as_str()))
            .map(|依存名| 違反::ファイル単位(パス.to_path_buf(), format!("白リスト外の依存: {依存名}")))
            .collect(),
        None => 依存名一覧
            .into_iter()
            .map(|依存名| {
                違反::ファイル単位(パス.to_path_buf(), format!("白リスト未登録クレート({クレート名})の依存: {依存名}"))
            })
            .collect(),
    }
}
