//! 型の定義に属する固有・トレイトの実装から、可変参照メソッドを探す工程。

use std::path::PathBuf;

use super::impl_header::implの見出しを読む;
use super::line_matching::型に属するimplの宣言か;
use super::syntax_checker::クレート構文検査;
use super::trait_implementation::トレイト実装型;
use super::type_definition::{固有implのファイル, 定義ブロックの結果};

impl クレート構文検査 {
    /// その型に属する固有・トレイトの実装に `&mut self` があるか。
    /// 定義に属するファイルだけを見て、別のモジュールの同名の型を混ぜない。
    pub fn 可変参照メソッドを含むか(&self, 型: &トレイト実装型) -> bool {
        let 定義ブロックの結果::見つかった { パス: 定義のパス, .. } = self.型の定義ブロック(型) else {
            return false;
        };
        let 属するか = |パス: &PathBuf, 行一覧: &[String]| (固有implのファイル { パス, 行一覧 }).定義を指すか(&型.型名, &定義のパス);
        self.ソース一覧.iter().filter(|(パス, 行一覧)| 属するか(パス, 行一覧)).any(|(_, 行一覧)| {
            行一覧
                .iter()
                .enumerate()
                .filter_map(|(開始, _)| implの見出しを読む(行一覧, 開始))
                .filter(|見出し| 型に属するimplの宣言か(&見出し.表記, &型.型名))
                .any(|見出し| 行一覧[見出し.本体の開始行..=見出し.本体の終了行].iter().any(|行| 行.contains("&mut self")))
        })
    }
}
