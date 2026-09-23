//! 原文からコード以外を落として、行ごとの並びへ写す工程。受け取るのは原文、返すのは行番号順のコードだけの行である。
//!
//! 属性や波括弧を数える検査がこれを使う。コメントの中の`#[cfg(test)]`や、文字列の中の波括弧を
//! コードとして数えないためである。行番号を保つのは、違反の報告が原文の行を指せるようにするためである。
//!
//! ブロックコメントを落とした位置の前後がどちらも空白でないコードなら、その位置に半角空白を1つ置く。rustc はコメントを字句の区切りとして読むため、
//! 置かないと`impl/**/丙`が`impl丙`という1つの語になり、`impl 初期化/* 注 */for 規則`を固有の実装と読み違えるためである。
//! 文字列と文字の位置には置かない。`include_bytes!("..")`の括弧の中が空であることを手がかりにする検査があるためである。
//! 原文の先頭のバイト順マーク(U+FEFF)は落とす。rustc と同じく原文の一部として読まないためであり、残すと1行目の宣言が行の頭に無いと読まれる。

use super::{字句の区分, 字句へ分ける};

pub fn コードだけの行一覧(内容: &str) -> Vec<String> {
    let 内容 = 内容.strip_prefix('\u{feff}').unwrap_or(内容);
    let mut 行一覧 = vec![String::new(); 内容.lines().count()];
    let mut 区切りを待つ行: Option<usize> = None;
    for 断片 in 字句へ分ける(内容) {
        match 断片.区分 {
            字句の区分::コード => {}
            字句の区分::ブロックコメント => {
                区切りを待つ行 = Some(断片.開始行 + 断片.中身.matches('\n').count());
                continue;
            }
            字句の区分::行コメント | 字句の区分::文字列リテラル | 字句の区分::文字リテラル => {
                区切りを待つ行 = None;
                continue;
            }
        }
        for (ずれ, 部分) in 断片.中身.split('\n').enumerate() {
            let Some(行) = 行一覧.get_mut(断片.開始行 + ずれ - 1) else {
                continue;
            };
            let 前が語か = 行.chars().next_back().is_some_and(|文字| !文字.is_whitespace());
            let 後ろが語か = 部分.chars().next().is_some_and(|文字| !文字.is_whitespace());
            if ずれ == 0 && 区切りを待つ行 == Some(断片.開始行) && 前が語か && 後ろが語か {
                行.push(' ');
            }
            行.push_str(部分);
        }
        区切りを待つ行 = None;
    }
    行一覧
}

#[cfg(test)]
mod tests {
    use super::コードだけの行一覧;

    #[test]
    fn ブロックコメントの前後の語を半角空白で区切る() {
        assert_eq!(コードだけの行一覧("impl/**/丙 {}\n"), vec!["impl 丙 {}"]);
        assert_eq!(コードだけの行一覧("impl 初期化/* 注 */for 規則 {}\n"), vec!["impl 初期化 for 規則 {}"]);
        assert_eq!(コードだけの行一覧("impl/*\n*/丙 {}\n"), vec!["impl", "丙 {}"]);
    }

    #[test]
    fn 空白の隣と文字列の位置には空白を置かない() {
        assert_eq!(コードだけの行一覧("let 値 = 1; /* 注 */\n"), vec!["let 値 = 1; "]);
        assert_eq!(コードだけの行一覧("let 値 = include_bytes!(\"a.bin\");\n"), vec!["let 値 = include_bytes!();"]);
        assert_eq!(コードだけの行一覧("let 値 = a/* 注 */\"x\"b;\n"), vec!["let 値 = ab;"]);
    }

    #[test]
    fn 先頭のバイト順マークを落とす() {
        assert_eq!(コードだけの行一覧("\u{feff}use a;\nstruct 甲;\n"), vec!["use a;", "struct 甲;"]);
    }
}
