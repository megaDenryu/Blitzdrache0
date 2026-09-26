//! 生成物の取り込みとは、Graphiteが利用者に書かせる `pub mod 名前 { include!("generated/名前.rs"); }` の形の、生成物を1つだけ取り込む波括弧付きのモジュールのことである。
//! 受け取るのは宣言のファイルの原文と `mod` の宣言の行番号、返すのは本体がこの形のときの `include!` の表記である。
//!
//! 本体が `include!` の1文だけでない波括弧付きのモジュールは取り込みと認めない。取り込みと認めた本体は設計関係の抽出が辿らないため、
//! 手書きのコードを同じ本体へ混ぜた形まで認めると、そのコードが抽出から黙って外れる。

use super::super::source_lexing::{コードだけの行一覧, 文字列リテラル一覧};

/// 本体を空白を除いてつないだとき、取り込みの形がとる唯一の並び。コードだけの行は文字列を落とすため括弧の中は空になる。
const 取り込みの本体: &str = "include!();";

/// 宣言の行の `{` から対応する `}` までの本体が `include!` の1文だけなら、その表記を返す。本体の中の文字列リテラルがちょうど1つであることも求める。
pub(super) fn 波括弧の中の取り込み先(原文: &str, 宣言の行番号: usize) -> Option<String> {
    let 行一覧 = コードだけの行一覧(原文);
    let 開始 = 宣言の行番号.checked_sub(1)?;
    let 開き = 行一覧.get(開始)?.find('{')?;
    let mut 本体 = String::new();
    let mut 深さ = 1usize;
    let mut 閉じた行 = None;
    for (添字, 行) in 行一覧.iter().enumerate().skip(開始) {
        let 読む部分 = if 添字 == 開始 { &行[開き + 1..] } else { 行.as_str() };
        for 文字 in 読む部分.chars() {
            深さ = match 文字 {
                '{' => 深さ + 1,
                '}' => 深さ - 1,
                _ => 深さ,
            };
            if 深さ == 0 {
                閉じた行 = Some(添字);
                break;
            }
            本体.push(文字);
        }
        if 閉じた行.is_some() {
            break;
        }
    }
    let 閉じた行 = 閉じた行?;
    if 本体.chars().filter(|文字| !文字.is_whitespace()).collect::<String>() != 取り込みの本体 {
        return None;
    }
    let 本体の範囲 = 開始 + 1..=閉じた行 + 1;
    match 文字列リテラル一覧(原文).into_iter().filter(|断片| 本体の範囲.contains(&断片.開始行)).collect::<Vec<_>>().as_slice() {
        [唯一] => Some(唯一.中身.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::波括弧の中の取り込み先;

    #[test]
    fn 生成物を取り込む形から表記を読む() {
        let 原文 = "#[allow(non_snake_case)]\npub mod 経路網 {\n    include!(\"generated/経路網.rs\");\n}\n";
        assert_eq!(波括弧の中の取り込み先(原文, 2), Some("generated/経路網.rs".to_string()));
        assert_eq!(波括弧の中の取り込み先("mod 網 { include!(\"generated/網.rs\"); }\n", 1), Some("generated/網.rs".to_string()));
    }

    #[test]
    fn 本体に手書きのコードが混ざれば取り込みと認めない() {
        let 原文 = "pub mod 経路網 {\n    include!(\"generated/経路網.rs\");\n    pub struct 手書き;\n}\n";
        assert_eq!(波括弧の中の取り込み先(原文, 1), None);
        assert_eq!(波括弧の中の取り込み先("mod 網 { include!(\"a.rs\"); include!(\"b.rs\"); }\n", 1), None);
        assert_eq!(波括弧の中の取り込み先("mod 網 { fn 何か() {} }\n", 1), None);
    }

    #[test]
    fn 閉じない本体と宣言でない行は読まない() {
        assert_eq!(波括弧の中の取り込み先("mod 網 {\n    include!(\"a.rs\");\n", 1), None);
        assert_eq!(波括弧の中の取り込み先("mod 網;\n", 1), None);
        assert_eq!(波括弧の中の取り込み先("mod 網;\n", 0), None);
    }
}
