//! 型の定義に属する固有・トレイトの実装から、可変参照メソッドを探す工程。

use std::path::PathBuf;

use super::line_matching::{implの見出しを読む, 型に属するimplの宣言か, 波括弧が閉じる行};
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
                .filter(|(見出し, _)| 型に属するimplの宣言か(見出し, &型.型名))
                .any(|(_, 本体の開始)| 行一覧[本体の開始..=波括弧が閉じる行(行一覧, 本体の開始)].iter().any(|行| 行.contains("&mut self")))
        })
    }
}
