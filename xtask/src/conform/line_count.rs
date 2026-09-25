//! 行数検査: .rs / .slang / .ts の各ファイルのコードの行を数える。
//! 数えるのはコードの行だけであり、空行とコメントだけの行は数えない。
//! Rustのファイルでは、利用者が手で書くGraphiteのマクロの本体の行を数えから除く(2026-09-26のオーナー裁定。Issue #187)。
//! 除いた行があれば、違反の説明に除いた行数を添える。数えた行数だけを見せると、原文の行数と食い違う理由が読み手に分からないためである。
//! 何行まで許すかの判定と、超過を許したファイルの台帳は `line_count_allowance` が持つ。
//! 参照: グローバルCLAUDE.md「1ファイル100行の原則と分割の質」の分割してよいかの判定、条2。

use std::path::Path;

use super::graphiteのコード::Graphiteのマクロの本体の行;
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
    let 本体の行数 = if パス.extension().is_some_and(|拡張子| 拡張子 == "rs") {
        graphiteのマクロの本体のコードの行を数える(内容)
    } else {
        0
    };
    let mut 違反一覧 = 数えた行数を台帳と突き合わせる(パス, 行数を数える(内容).saturating_sub(本体の行数));
    if 本体の行数 > 0 {
        for 違反 in &mut 違反一覧 {
            違反.説明.push_str(&format!("(Graphiteのマクロの本体の{本体の行数}行を除いて数えた)"));
        }
    }
    違反一覧
}

fn graphiteのマクロの本体のコードの行を数える(内容: &str) -> usize {
    let 本体 = Graphiteのマクロの本体の行::原文から読む(内容);
    行ごとの内訳(内容).iter().enumerate().filter(|(添字, 内訳)| 内訳.コードを含む && 本体.含むか(添字 + 1)).count()
}

#[cfg(test)]
#[path = "line_count/graphiteのマクロの本体を除く試験.rs"]
mod graphiteのマクロの本体を除く試験;

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
