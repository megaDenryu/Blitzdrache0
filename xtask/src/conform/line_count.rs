//! 行数検査: .rs / .slang / .ts の各ファイルが100行以内か確認する。
//! 数えるのはコードの行だけであり、空行とコメントだけの行は数えない。
//! 参照: グローバルCLAUDE.md「1ファイル100行の原則と分割の質」の分割してよいかの判定、条2。

use std::path::Path;

use super::source_lexing::{字句の区分, 字句の断片, 字句へ分ける};
use super::violation::違反;

pub const 上限行数: usize = 100;

/// コードを含む行の数を返す。行末に書いたコメントはコードの行として数える。
pub fn 行数を数える(内容: &str) -> usize {
    let 原文: Vec<char> = 内容.replace("\r\n", "\n").chars().collect();
    let mut コードを含む行 = vec![false; 内容.lines().count()];
    for 断片 in 字句へ分ける(内容) {
        if matches!(断片.区分, 字句の区分::行コメント | 字句の区分::ブロックコメント) {
            continue;
        }
        印を付ける(&原文, &断片, &mut コードを含む行);
    }
    コードを含む行.into_iter().filter(|印| *印).count()
}

/// 断片が占める区間を原文の上で辿り、空白以外の文字が在る行へ印を付ける。
/// 中身でなく原文の区間を辿るのは、文字列リテラルの中身が逃がし記号を復号したものであり、
/// `\n`の逃がし1つで以降の行番号がずれるためである。
fn 印を付ける(原文: &[char], 断片: &字句の断片, コードを含む行: &mut [bool]) {
    let mut 行番号 = 断片.開始行;
    for 添字 in 断片.開始位置..断片.終了位置 {
        match 原文.get(添字) {
            Some('\n') => 行番号 += 1,
            Some(文字) if !文字.is_whitespace() => {
                if let Some(印) = コードを含む行.get_mut(行番号 - 1) {
                    *印 = true;
                }
            }
            _ => {}
        }
    }
}

pub fn 行数超過か(行数: usize) -> bool {
    行数 > 上限行数
}

/// ts-rsが生成した型契約(手編集禁止)は行数検査の対象外にする。
/// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`層の定義「型契約(生成)」。
pub fn 生成ファイルか(パス: &Path) -> bool {
    パス.components().any(|部分| 部分.as_os_str() == "生成")
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let 行数 = 行数を数える(内容);
    if 行数超過か(行数) {
        vec![違反::ファイル単位(パス.to_path_buf(), format!("コードの行が{行数}行"))]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 空文字列は0行() {
        assert_eq!(行数を数える(""), 0);
    }

    #[test]
    fn 末尾改行なしでも行数を数えられる() {
        assert_eq!(行数を数える("let a = 1;\nlet b = 2;"), 2);
    }

    #[test]
    fn 空行は数えない() {
        assert_eq!(行数を数える("let a = 1;\n\n\nlet b = 2;\n"), 2);
    }

    #[test]
    fn コメントだけの行は数えない() {
        assert_eq!(行数を数える("//! 見出し\n\n/// 説明\nlet a = 1;\n"), 1);
    }

    #[test]
    fn 複数行にまたがるブロックコメントは数えない() {
        assert_eq!(行数を数える("let a = 1;\n/* 一行目\n二行目\n三行目 */\nlet b = 2;\n"), 2);
    }

    #[test]
    fn 行末のコメントが付いた行はコードの行として数える() {
        assert_eq!(行数を数える("let a = 1; // 説明\nlet b = 2; // 説明\n"), 2);
    }

    #[test]
    fn 文字列リテラルだけの行もコードの行として数える() {
        assert_eq!(行数を数える("const A: [&str; 2] = [\n    \"//まぎらわしい綴り\"\n];\n"), 3);
    }

    #[test]
    fn 文字列リテラルの中の改行の逃がしで行がずれない() {
        let 原文 = concat!(r#"let a = "一行目\n二行目";"#, "\n// 説明\nlet b = 2;\n");
        assert_eq!(行数を数える(原文), 2);
    }

    #[test]
    fn 上限を超えたら違反と判定する() {
        assert!(!行数超過か(上限行数));
        assert!(行数超過か(上限行数 + 1));
    }

    #[test]
    fn 生成ディレクトリ配下は生成ファイルと判定する() {
        assert!(生成ファイルか(Path::new("editor_web/src/生成/編集資源契約.ts")));
        assert!(!生成ファイルか(Path::new("editor_web/src/入り口/エディター外殻.ts")));
    }
}
