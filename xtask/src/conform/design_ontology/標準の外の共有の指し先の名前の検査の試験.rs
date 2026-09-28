//! 標準の外の共有の指し先の名前の検査の試験。純粋データ規約の対象の型を持つクレートで、`Arc`・`Rc`・`Weak` の名前を標準の表記以外から持ち込む `use` と、
//! その名前を含む読み切れない `use` 文と、その名前の型の定義が違反になることと、標準の表記の取り込みと保証範囲の外の形が違反にならないことを固定する。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 取り込みの違反: &str = "共有の指し先の名前(`Arc`・`Rc`・`Weak`)を標準の表記";
const 読み切れない取り込みの違反: &str = "を含む `use` 文を読み切れない";
const 定義の違反: &str = "共有の指し先の名前(`Arc`・`Rc`・`Weak`)で型を定義しない";

const 共有される地図: &str = "pub struct 地図 {\n    つながり: Vec<u8>,\n}\nimpl M共有される不変データ for 地図 {}\n";

// 前置きの後に、地図を修飾の無い `Arc<地図>` のフィールドで持つ規則を置いた内容。
fn 前置きの後に地図を持つ規則(前置き: &str) -> String {
    format!("{前置き}{共有される地図}pub struct 規則 {{\n    地図: Arc<地図>,\n}}\nimpl M不変データ for 規則 {{}}\nimpl M規則 for 規則 {{}}\n")
}

fn 違反の説明一覧(前置き: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &前置きの後に地図を持つ規則(前置き))])
}

#[test]
fn 構文解析_共有の指し先の名前を標準の外から取り込むと違反になる() {
    for 前置き in [
        "use crate::自前::Arc;\n",
        "pub(in crate::x) use crate::自前::{Arc, 設定};\n",
        "use crate::自前::共有 as Arc;\n",
        "use crate::自前::Rc;\n",
        "use std::sync::{\n    Mutex,\n    Weak as Rc,\n};\n",
    ] {
        let 説明一覧 = 違反の説明一覧(前置き);
        assert!(説明一覧.iter().any(|説明| 説明.contains(取り込みの違反)), "{前置き}: {説明一覧:?}");
    }
}

#[test]
fn 構文解析_共有の指し先の名前を含む読み切れない取り込みの文は違反になる() {
    for 前置き in ["use crate::自前::{Arc}::甲;\n", "use crate::自前::{Arc, $名前};\n"] {
        let 説明一覧 = 違反の説明一覧(前置き);
        assert_eq!(説明一覧.len(), 1, "{前置き}: {説明一覧:?}");
        assert!(説明一覧[0].contains(読み切れない取り込みの違反), "{説明一覧:?}");
    }
}

#[test]
fn 構文解析_共有の指し先の名前で型を定義すると違反になる() {
    for 前置き in ["pub struct Arc<T>(T);\n", "#[derive(Clone)] pub(crate) enum Rc {\n    甲,\n}\n", "union Weak {\n    値: u8,\n}\n"] {
        let 説明一覧 = 違反の説明一覧(前置き);
        assert_eq!(説明一覧.len(), 1, "{前置き}: {説明一覧:?}");
        assert!(説明一覧[0].contains(定義の違反), "{説明一覧:?}");
    }
}

#[test]
fn 構文解析_標準の表記の取り込みと名前の一部だけが一致する定義は違反にならない() {
    let 前置き = "use std::sync::{Arc, Mutex};\nuse alloc::sync::Weak;\nuse std::rc::{self, Rc};\nuse crate::設定::Arcの設定;\npub struct Arcの設定;\n";
    assert!(違反の説明一覧(前置き).is_empty(), "{:?}", 違反の説明一覧(前置き));
}

#[test]
fn 構文解析_保証範囲の外_globの取り込みと対象の型を持たないクレートの取り込みは違反にならない() {
    assert!(違反の説明一覧("use crate::自前::*;\n").is_empty());
    let 別のクレート = ソース("crates/b/src/lib.rs", "use crate::自前::Arc;\npub struct Rc;\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![別のクレート]).is_empty());
}
