//! 宣言の並びの検査: 構造体・列挙の宣言ブロックの中に、コメントだけの行が無いことを確かめる。
//!
//! 名前と型が縦に揃っていること自体が読みやすさの源であり、宣言と宣言の間へコメントの行が入ると
//! 読み手は宣言を目で拾い直すことになる。説明は宣言と同じ行の末尾へ書く。
//! 参照: グローバルCLAUDE.md「コメント」、CLAUDE.md「コメント」。

use std::path::Path;

use super::source_lexing::{コードだけの行一覧, 行ごとの内訳};
use super::violation::違反;

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let 内訳一覧 = 行ごとの内訳(内容);
    let コード行一覧 = コードだけの行一覧(内容);
    let mut 違反一覧 = Vec::new();
    let mut 深さ = 0usize;
    for (添字, コード行) in コード行一覧.iter().enumerate() {
        if 深さ == 0 {
            if 宣言ブロックの開きか(コード行) {
                深さ = 1;
            }
            continue;
        }
        深さ = (深さ + コード行.matches('{').count()).saturating_sub(コード行.matches('}').count());
        if 深さ == 0 {
            continue;
        }
        if 宣言に付かないコメントの行か(&内訳一覧, &コード行一覧, 添字) {
            違反一覧.push(違反::行単位(
                パス.to_path_buf(),
                添字 + 1,
                "宣言の間にコメントだけの行が在る。説明は宣言と同じ行の末尾へ書く".to_string(),
            ));
        }
    }
    違反一覧
}

/// 構造体・列挙の宣言が波括弧を開く行かを判定する。タプル構造体と型別名は波括弧を持たないため掛からない。
fn 宣言ブロックの開きか(コード行: &str) -> bool {
    let 綴り = コード行.trim_start();
    let 綴り = 綴り
        .strip_prefix("pub")
        .map_or(綴り, |残り| 残り.trim_start_matches(|文字| 文字 != ' ').trim_start());
    (綴り.starts_with("struct ") || 綴り.starts_with("enum ")) && コード行.trim_end().ends_with('{')
}

/// コメントだけの行のうち、直後に宣言が続くものを違反とする。直後が空行や閉じ括弧のものは宣言の並びを割らない。
fn 宣言に付かないコメントの行か(
    内訳一覧: &[super::source_lexing::行の内訳], コード行一覧: &[String], 添字: usize
) -> bool {
    let Some(内訳) = 内訳一覧.get(添字) else { return false };
    if 内訳.コードを含む || !内訳.コメントを含む {
        return false;
    }
    let mut 次 = 添字 + 1;
    while 内訳一覧.get(次).is_some_and(|次の内訳| !次の内訳.コードを含む && 次の内訳.コメントを含む) {
        次 += 1;
    }
    コード行一覧.get(次).is_some_and(|行| !行.trim().is_empty() && 行.trim() != "}")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 違反の行番号(内容: &str) -> Vec<usize> {
        検査する(Path::new("試験.rs"), 内容).into_iter().filter_map(|違反| 違反.行番号).collect()
    }

    #[test]
    fn フィールドの上のコメントを違反とする() {
        assert_eq!(違反の行番号("struct 型 {\n    /// 説明\n    値: u32,\n}\n"), vec![2]);
    }

    #[test]
    fn 行末のコメントは違反にしない() {
        assert!(違反の行番号("struct 型 {\n    値: u32, // 説明\n}\n").is_empty());
    }

    #[test]
    fn 列挙の枝の上のコメントを違反とする() {
        assert_eq!(違反の行番号("enum 型 {\n    甲,\n    // 説明\n    乙,\n}\n"), vec![3]);
    }

    #[test]
    fn 複数行のコメントは先頭の1件だけを報告する() {
        assert_eq!(違反の行番号("struct 型 {\n    /// 一行目\n    /// 二行目\n    値: u32,\n}\n"), vec![2, 3]);
    }

    #[test]
    fn 型やモジュールの上のコメントは違反にしない() {
        assert!(違反の行番号("//! 見出し\n\n/// 型の説明\nstruct 型 {\n    値: u32,\n}\n").is_empty());
    }

    #[test]
    fn 関数の中のコメントは違反にしない() {
        assert!(違反の行番号("fn 手続き() {\n    // 段落の説明\n    let a = 1;\n}\n").is_empty());
    }

    #[test]
    fn 構造体リテラルの中のコメントは違反にしない() {
        assert!(違反の行番号("fn 手続き() -> 型 {\n    型 {\n        // 説明\n        値: 1,\n    }\n}\n").is_empty());
    }

    #[test]
    fn 閉じ括弧の直前のコメントは宣言の並びを割らないため違反にしない() {
        assert!(違反の行番号("struct 型 {\n    値: u32,\n    // 補足\n}\n").is_empty());
    }
}
