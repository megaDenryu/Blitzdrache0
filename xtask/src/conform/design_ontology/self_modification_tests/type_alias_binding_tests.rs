//! 自己変更の禁止の試験のうち、型の別名を通した実装の結び付けを確かめるもの。glob で取り込んだモジュールの `pub use … as 別名`・別のクレートの `pub type 別名`・同じ名前の複数の `type` の別名・
//! `mod` の本体の中の `type` の別名を、自己変更の根拠を持つなら違反にし、実装の本体の中の関連型(`type 出力 = 規則;`)を型の別名として拾わないことを固定する。
//! 別名の表をクレートごとに1つの右辺で持つ形は、後に読んだ同名の別名で先の別名を上書きし、別のクレートの `pub type` を通した実装を黙って外していた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 結び付けられない違反: &str = "M不変データ `規則` と同じ名前を対象にする";
const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数: &str = "{\n    fn 変える(&mut self) {}\n}\n";

fn 違反が1件だけあり結び付けられない(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(結び付けられない違反), "{説明一覧:?}");
}

#[test]
fn globで取り込んだモジュールのuseの別名を通した実装は違反になる() {
    違反が1件だけあり結び付けられない(vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/m.rs", "pub use crate::x::規則 as 別名;\n"),
        ソース("crates/a/src/y.rs", &format!("use crate::m::*;\nimpl 別名 {可変の関数}")),
    ]);
}

#[test]
fn 別のクレートのpubなtypeの別名を通した実装は違反になる() {
    違反が1件だけあり結び付けられない(vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/lib.rs", "pub type 別名 = x::規則;\n"),
        ソース("crates/b/src/y.rs", &format!("use a::別名;\nimpl 別名 {可変の関数}")),
    ]);
}

#[test]
fn 同じ名前の別名が複数あってもどれかが型名を指せば違反になる() {
    違反が1件だけあり結び付けられない(vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/y.rs", &format!("type 別名 = crate::x::規則;\nimpl 別名 {可変の関数}")),
        ソース("crates/a/src/z.rs", "pub struct 他;\ntype 別名 = 他;\n"),
    ]);
    違反が1件だけあり結び付けられない(vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/y.rs", &format!("mod 内 {{\n    pub type 別名 = crate::x::規則;\n}}\nimpl 内::別名 {可変の関数}")),
    ]);
}

#[test]
fn 実装の本体の中の関連型は型の別名として扱わない() {
    let 内容 = format!("pub struct 出力;\npub struct 他;\npub trait 変換 {{\n    type 出力;\n}}\nimpl 変換 for 他 {{\n    type 出力 = crate::x::規則;\n}}\nimpl 出力 {可変の関数}");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/y.rs", &内容)]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
