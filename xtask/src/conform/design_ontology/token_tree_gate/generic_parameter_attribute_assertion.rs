//! `impl` と `type` の型引数に属性(`impl<#[cfg(..)] T: 境界> 変更 for T`・`type 同じ<#[cfg(..)] T = 規則> = T;`)を書かないことを確かめる工程。
//! 受け取るのは字句の木の一覧(違反一覧を作るとき)か、字句の木の並びと識別子の添字(字句の木を数えるとき)、返すのは違反一覧か、その識別子の宣言が型引数に属性を持つかである。
//! 型引数の属性は `cfg` で型引数そのものを消したり足したりでき、検査器はその真偽を評価できない。読み口と字句の木は属性を読み飛ばして型引数の名前を読むが、どの構成の宣言を読んだかを決められないため、宣言そのものを禁じる。
//! 以前は読み口と字句の木がどちらも「各引数の先頭の識別子」を名前にしており、属性の付いた `T` を両方が落として全称の実装の判定を抜けていた。
//! 字句の木で数えるのは、属性と型引数の間の改行やコメントで行単位の照合をすり抜けないためである。数えるのは位置によらず `impl <` と `type 名前 <` のすべてである。走査範囲の現状は0件である。

use std::path::Path;

use proc_macro2::{Delimiter, TokenTree};

use super::super::super::violation::違反;
use super::angle_bracket_scan::{最上位の位置, 記号か};
use super::token_tree_index::字句の木の一覧;

/// 並びの添字の識別子が `impl` か `type 名前` で、その直後の型引数の山括弧の中に外側の属性(`#` と角括弧の群)があるか。
pub fn 型引数に属性を持つ宣言か(字句一覧: &[TokenTree], 添字: usize) -> bool {
    let 開く位置 = match (字句一覧.get(添字), 字句一覧.get(添字 + 1)) {
        (Some(TokenTree::Ident(語)), _) if 語 == "impl" => 添字 + 1,
        (Some(TokenTree::Ident(語)), Some(TokenTree::Ident(_))) if 語 == "type" => 添字 + 2,
        _ => return false,
    };
    let 後ろ = 字句一覧.get(開く位置..).unwrap_or_default();
    if !後ろ.first().is_some_and(|字句| 記号か(字句, '<')) {
        return false;
    }
    let 閉じる位置 = 最上位の位置(後ろ, |位置| 位置 > 0 && 記号か(&後ろ[位置], '>')).unwrap_or(後ろ.len());
    後ろ
        .get(1..閉じる位置)
        .unwrap_or_default()
        .windows(2)
        .any(|対| 記号か(&対[0], '#') && matches!(&対[1], TokenTree::Group(群) if 群.delimiter() == Delimiter::Bracket))
}

/// 型引数に属性を持つ `impl` と `type` の違反一覧。並びはパスの順、ファイルの中では原文の順である。字句の木へ変えられなかったファイルは、その違反が別に出るため数えない。
pub fn 型引数の属性の違反一覧(字句の木: &字句の木の一覧) -> Vec<違反> {
    字句の木
        .数えたファイルの一覧()
        .flat_map(|(パス, 数えた)| 数えた.型引数に属性を持つ宣言の行一覧.iter().map(move |行番号| 型引数の属性の違反(パス, *行番号)))
        .collect()
}

fn 型引数の属性の違反(パス: &Path, 行番号: usize) -> 違反 {
    let 説明 = "設計オントロジー: `impl` か `type` の型引数に属性を書いている。型引数の属性は `cfg` で型引数そのものを消したり足したりでき、検査器はその真偽を評価できず、全称の実装の判定と名前の閉包が読む型引数を決められないため違反にする。構成で分けるなら宣言ごと属性を付けて書き分ける".to_string();
    違反::行単位(パス.to_path_buf(), 行番号, 説明)
}

#[cfg(test)]
mod tests {
    use super::super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};

    fn 属性の違反の件数(本文: &str) -> usize {
        正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)]).iter().filter(|説明| 説明.contains("型引数に属性を書いている")).count()
    }

    #[test]
    fn 実装と型の別名の型引数の属性を違反にし属性の無い型引数と宣言の属性は違反にしない() {
        assert_eq!(属性の違反の件数("impl<#[cfg(test)] T: Clone> 甲 for T {}\n"), 1);
        assert_eq!(属性の違反の件数("pub type 同じ<'a, #[cfg(test)]\nT> = &'a T;\n"), 1);
        assert_eq!(属性の違反の件数("impl<T: Into<Vec<u8>>, const N: usize> 甲 for 乙<T, N> {}\n#[cfg(test)]\npub type 丙<T> = Vec<T>;\n"), 0);
    }
}
