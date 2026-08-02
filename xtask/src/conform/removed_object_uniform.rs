//! 段3で廃止した描画対象ごとのシェーダー定数の経路が書き戻されていないことの検査。受け取るのは無し(対象の語の表がここにある)、
//! 返すのは廃止した語を持つファイルの違反一覧である。
//!
//! 廃止したのは、個体変換112バイトと材質係数32バイトを1つの144バイトのシェーダー定数へ詰めた構造と、その先頭を
//! 個体変換1件として別名で読むディスクリプタの結び方である。この形はコンパイルの通る形で書き戻せてしまい、
//! 書き戻しても絵は同じに見えるため、名前の不在を機械的に守る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」)。
//!
//! 走査対象をcratesとshadersに限るのは、この検査自身のソースが対象の語を持つためである。

use std::path::{Path, PathBuf};

use super::violation::違反;
use crate::file_scan;

const 走査対象ディレクトリ一覧: [&str; 2] = ["crates", "shaders"];
const 走査対象拡張子一覧: [&str; 2] = ["rs", "slang"];

/// 書き戻しを禁じる語。GPU側の構造体名と変数名、CPU側の資源名とモジュール名、単一個体の別名経路が持っていた型名である。
const 廃止語一覧: [&str; 5] = [
    "ObjectUniform",
    "objectUniform",
    "object_uniform",
    "描画対象シェーダー定数",
    "個体変換の出どころ",
];

pub fn 全ファイルを検査する() -> Result<Vec<違反>, String> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&走査対象ディレクトリ一覧, &走査対象拡張子一覧)
        .map_err(|誤り| format!("廃止語検査のファイル走査に失敗した: {誤り}"))?;
    let mut 違反一覧 = Vec::new();
    for パス in &ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        違反一覧.extend(ファイル1つを検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn ファイル1つを検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        for 語 in 廃止語一覧 {
            if 行.contains(語) {
                違反一覧.push(違反::行単位(
                    PathBuf::from(パス),
                    行番号 + 1,
                    format!("段3で廃止した描画対象ごとのシェーダー定数の語({語})を書き戻している"),
                ));
            }
        }
    }
    違反一覧
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 廃止した構造体名を書き戻した行を違反にする() {
        let 違反一覧 = ファイル1つを検査する(Path::new("shaders/scene.slang"), "struct ObjectUniform\n");
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 単一個体の別名経路の型名を書き戻した行を違反にする() {
        let 違反一覧 = ファイル1つを検査する(Path::new("crates/blitz_render/src/新規.rs"), "enum 個体変換の出どころ {\n");
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 現行の名前は違反にしない() {
        let 違反一覧 = ファイル1つを検査する(Path::new("shaders/material_record.slang"), "struct MaterialRecord\n");
        assert!(違反一覧.is_empty());
    }
}
