//! 行数検査: .rs / .slang / .ts の各ファイルのコードの行を数える。
//! 数えるのはコードの行だけであり、空行とコメントだけの行は数えない。
//! 何行まで許すかの判定と、超過を許したファイルの台帳は `line_count_allowance` が持つ。
//! 参照: グローバルCLAUDE.md「1ファイル100行の原則と分割の質」の分割してよいかの判定、条2。

use std::path::Path;

use super::line_count_allowance::数えた行数を台帳と突き合わせる;
use super::source_lexing::行ごとの内訳;
use super::violation::違反;

/// コードを含む行の数を返す。行末に書いたコメントはコードの行として数える。
pub fn 行数を数える(内容: &str) -> usize {
    行ごとの内訳(内容).into_iter().filter(|内訳| 内訳.コードを含む).count()
}

/// ts-rsが生成した型契約(手編集禁止)は行数検査の対象外にする。
/// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`層の定義「型契約(生成)」。
pub fn 生成ファイルか(パス: &Path) -> bool {
    パス.components().any(|部分| 部分.as_os_str() == "生成")
}

pub fn 行数の上限超過を検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    数えた行数を台帳と突き合わせる(パス, 行数を数える(内容), 内容)
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
        assert_eq!(行数を数える("const A: [&str; 2] = [\n    \"//まぎらわしい文字列\"\n];\n"), 3);
    }

    #[test]
    fn 文字列リテラルの中の改行の逃がしで行がずれない() {
        let 原文 = concat!(r#"let a = "一行目\n二行目";"#, "\n// 説明\nlet b = 2;\n");
        assert_eq!(行数を数える(原文), 2);
    }

    #[test]
    fn 生成ディレクトリ配下は生成ファイルと判定する() {
        assert!(生成ファイルか(Path::new("editor_web/src/生成/編集資源契約.ts")));
        assert!(!生成ファイルか(Path::new("editor_web/src/入り口/エディター外殻.ts")));
    }
}
