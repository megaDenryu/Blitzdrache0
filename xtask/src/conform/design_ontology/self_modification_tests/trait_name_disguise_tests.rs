//! 自己変更の禁止の試験のうち、走査範囲のトレイトが prelude・依存クレートの名前を名乗る形と、同名のトレイトの宣言のクレートの区別を確かめるもの。
//! prelude の一覧を名前だけで当てる形は、`pub use crate::t::可変化 as Default;` を glob か2段の再公開で取り込んだ `impl Default for 規則` を通していた。
//! 索引を名前だけで引く形は、別のクレートの無関係な同名の宣言が自己変更を与える関数を持つとき、その名前のトレイトの実装を偽の違反にしていた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";

fn 違反が1件だけある(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
}

#[test]
fn preludeの名前を名乗らせた別名をglobか2段の再公開で取り込んだ実装は違反になる() {
    let 宣言 = || ソース("crates/a/src/t.rs", "pub trait 可変化 {\n    fn 変える(&mut self) {}\n}\n");
    let 別名 = || ソース("crates/a/src/m.rs", "pub use crate::t::可変化 as Default;\n");
    違反が1件だけある(vec![宣言(), 別名(), ソース("crates/a/src/x.rs", &format!("use crate::m::*;\n{定義}impl Default for 規則 {{}}\n"))]);
    let 再公開 = || ソース("crates/a/src/n.rs", "pub use crate::m::Default;\n");
    違反が1件だけある(vec![宣言(), 別名(), 再公開(), ソース("crates/a/src/x.rs", &format!("use crate::n::Default;\n{定義}impl Default for 規則 {{}}\n"))]);
    let globの再公開 = ソース("crates/a/src/n.rs", "pub use crate::m::*;\n");
    違反が1件だけある(vec![宣言(), 別名(), globの再公開, ソース("crates/a/src/x.rs", &format!("use crate::n::*;\n{定義}impl Default for 規則 {{}}\n"))]);
}

#[test]
fn 依存クレートの名前を名乗らせたモジュールの別名を通した実装は違反になる() {
    let 別名 = ソース("crates/blitz_esca/src/m.rs", "pub use crate::t as thiserror;\n");
    違反が1件だけある(vec![別名, ソース("crates/blitz_esca/src/x.rs", &format!("use crate::m::*;\n{定義}impl thiserror::未知 for 規則 {{}}\n"))]);
}

#[test]
fn preludeの名前で走査範囲に宣言したトレイトは索引の宣言で判定する() {
    違反が1件だけある(vec![ソース("crates/a/src/x.rs", &format!("{定義}trait Default {{\n    fn 変える(&mut self) {{}}\n}}\nimpl Default for 規則 {{}}\n"))]);
}

#[test]
fn 普通のpreludeのトレイトの実装とderiveは違反にならない() {
    let 内容 = format!("{定義}impl Default for 規則 {{\n    fn default() -> Self {{\n        Self\n    }}\n}}\n#[derive(Clone, Default)]\npub struct 設定;\nimpl M不変データ for 設定 {{}}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/m.rs", "pub use crate::t::読む as 別の名前;\n"), ソース("crates/a/src/x.rs", &内容)]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

#[test]
fn 別のクレートの同名の宣言は実装が取り込んでいなければ参照しない() {
    let 可変の宣言 = || ソース("crates/blitz_render/src/u.rs", "pub trait 更新 {\n    fn f(&mut self);\n}\n");
    let 読むだけの宣言 = || ソース("crates/blitz_esca/src/t.rs", "pub trait 更新 {\n    fn 読む(&self) {}\n}\n");
    let 旅行者 = "pub struct 旅行者;\nimpl M不変データ for 旅行者 {}\n";
    for 取り込み in ["", "use super::*;\n", "use crate::t::更新;\n"] {
        let 実装 = ソース("crates/blitz_esca/src/x.rs", &format!("{取り込み}{旅行者}impl 更新 for 旅行者 {{}}\n"));
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![可変の宣言(), 読むだけの宣言(), 実装]);
        assert!(説明一覧.is_empty(), "{取り込み}: {説明一覧:?}");
    }
    違反が1件だけある(vec![可変の宣言(), 読むだけの宣言(), ソース("crates/blitz_esca/src/x.rs", &format!("use blitz_render::更新;\n{旅行者}impl 更新 for 旅行者 {{}}\n"))]);
}
