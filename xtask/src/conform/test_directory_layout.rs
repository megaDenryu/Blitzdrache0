//! 統合試験の置き場の検査。クレートの`tests/`の直下に`.rs`ファイルを置くことを禁じ、
//! 分割の単位をフォルダ(`tests/<名前>/main.rs` + `mod`の列)にすることを強制する。
//!
//! Cargoは`tests/`の直下の`.rs`ファイル1つを、独立した実行ファイル1本として組み立てる。
//! 実行ファイル1本ごとに、それが使う外部の部品すべてぶんのデバッグ情報が付くため、
//! ファイルを責務で分けるほど成果物が増える。ここだけ分割の費用が他と違う。
//! 参照: CLAUDE.md「ファイル・関数の分割」。

use std::path::Path;

use super::violation::違反;

/// クレートの`tests/`の直下に置いた`.rs`ファイルか。フォルダの下へ置いた試験は対象外である。
pub fn 試験の直置きか(パス: &Path) -> bool {
    let 部分一覧: Vec<&str> = パス.components().filter_map(|部分| 部分.as_os_str().to_str()).collect();
    let Some(起点) = 部分一覧.iter().position(|部分| *部分 == "crates") else {
        return false;
    };
    部分一覧.len() == 起点 + 4 && 部分一覧[起点 + 2] == "tests" && パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("rs")
}

pub fn 検査する(パス: &Path) -> Vec<違反> {
    if 試験の直置きか(パス) {
        vec![違反::ファイル単位(
            パス.to_path_buf(),
            "tests/の直下の.rsファイルは実行ファイル1本になる。tests/<名前>/main.rsのフォルダ形式へ移す".to_string(),
        )]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 直下のファイルを違反と判定する() {
        assert!(試験の直置きか(Path::new("crates/editor_server/tests/music_routes.rs")));
        assert_eq!(検査する(Path::new("crates/editor_server/tests/music_routes.rs")).len(), 1);
    }

    #[test]
    fn フォルダの下の試験は違反にしない() {
        assert!(!試験の直置きか(Path::new("crates/editor_server/tests/integration/main.rs")));
        assert!(!試験の直置きか(Path::new(
            "crates/blitz_asset_compiler/tests/integration/height_field_roundtrip/mod.rs"
        )));
        assert!(検査する(Path::new("crates/editor_server/tests/integration/main.rs")).is_empty());
    }

    #[test]
    fn srcの下と拡張子違いは違反にしない() {
        assert!(!試験の直置きか(Path::new("crates/editor_server/src/lib.rs")));
        assert!(!試験の直置きか(Path::new("crates/editor_server/tests/README.md")));
        assert!(!試験の直置きか(Path::new("xtask/src/conform/mod.rs")));
    }
}
