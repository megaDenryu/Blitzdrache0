//! 共有の指し先の別名の検査の試験。純粋データ規約の対象の型を持つクレートで `Arc`・`Rc`・`Weak` を `use … as` や `type` の別名にすると違反になることと、
//! 検査が読まない形(対象の型を持たないクレートの別名、マーカーの無い型を間に挟んだ指し先)が保証範囲の外として違反にならないことを固定する。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 別名の違反: &str = "共有の指し先(`Arc`・`Rc`・`Weak`)を `use … as` や `type` の別名にしない";

const 共有される地図: &str = "pub struct 地図 {\n    つながり: Vec<u8>,\n}\nimpl M共有される不変データ for 地図 {}\n";

// 前置き(別名の宣言)の後に、地図を `型の表記` のフィールドで持つ規則を置いた内容。
fn 別名を置いて地図を持つ規則(前置き: &str, 型の表記: &str) -> String {
    format!("{前置き}{共有される地図}pub struct 規則 {{\n    地図: {型の表記},\n}}\nimpl M不変データ for 規則 {{}}\nimpl M規則 for 規則 {{}}\n")
}

#[test]
fn 構文解析_共有の指し先をuseのasで別名にすると違反になる() {
    for 前置き in ["use std::rc::Rc as 共有;\n", "use std::sync::Arc as 共有;\n", "use std::sync::{\n    Weak as 共有,\n};\n"] {
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &別名を置いて地図を持つ規則(前置き, "共有<地図>"))]);
        assert_eq!(説明一覧.len(), 1, "{前置き}: {説明一覧:?}");
        assert!(説明一覧[0].contains(別名の違反), "{説明一覧:?}");
    }
}

#[test]
fn 構文解析_共有の指し先を右辺に持つtypeの別名は別のファイルに置いても違反になる() {
    for 別名 in ["type 共有の地図 = Rc<地図>;\n", "pub(crate) type 共有<T> = std::sync::Arc<T>;\n", "pub type 共有の地図 =\n    Arc<地図>;\n"] {
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &別名を置いて地図を持つ規則("", "共有の地図")), ソース("crates/a/src/y.rs", 別名)]);
        assert_eq!(説明一覧.len(), 1, "{別名}: {説明一覧:?}");
        assert!(説明一覧[0].contains(別名の違反), "{説明一覧:?}");
    }
}

#[test]
fn 構文解析_可視性の修飾をどう書いても別名の文として読む() {
    for 前置き in [
        "pub(in crate::x) use std::rc::Rc as 共有;
",
        "pub(self) type 共有<T> = Rc<T>;
",
        "#[allow(unused)] pub(in super::y) type 共有<T> = Rc<T>;
",
    ] {
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &別名を置いて地図を持つ規則(前置き, "共有<地図>"))]);
        assert_eq!(説明一覧.len(), 1, "{前置き}: {説明一覧:?}");
        assert!(説明一覧[0].contains(別名の違反), "{説明一覧:?}");
    }
}

#[test]
fn 構文解析_別名にしない取り込みと名前の一部だけが一致する別名は違反にならない() {
    let 前置き = "use std::sync::Arc;\nuse crate::設定::Arcの設定 as 設定;\ntype 地図の件数 = u8;\n";
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &別名を置いて地図を持つ規則(前置き, "Arc<地図>"))]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

#[test]
fn 構文解析_保証範囲の外_対象の型を持たないクレートの別名とマーカーの無い型を挟んだ指し先は違反にならない() {
    let 別のクレート = ソース("crates/b/src/lib.rs", "pub type 共有の地図 = std::rc::Rc<u8>;\nuse std::rc::Rc as 共有;\n");
    let 別名を使う = ソース("crates/a/src/x.rs", &別名を置いて地図を持つ規則("", "b::共有の地図"));
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![別のクレート, 別名を使う]).is_empty());
    let 挟む = format!("pub struct 箱 {{\n    中: Rc<地図>,\n}}\n{}", 別名を置いて地図を持つ規則("", "箱"));
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &挟む)]).is_empty());
}
