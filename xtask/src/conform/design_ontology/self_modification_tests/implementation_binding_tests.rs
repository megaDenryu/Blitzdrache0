//! 自己変更の禁止の試験のうち、実装の対象の型を定義へ結び付けることを確かめるもの。再公開を1段たどる取り込み・glob の取り込み・丸括弧で囲んだ型は定義へ結び付け、
//! 結び付けられないパス・同じクレートの解けない glob・`use … as` と `type` の別名は、自己変更の根拠を持つなら黙って外さず違反にすることを固定する。
//! 型名が一致しても定義を指すと示せない実装を黙って外す形は、`crate::` から再公開を経由して取り込んだ `impl 旅行者の現在地` の可変の関数を通していた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 結び付けられない違反: &str = "M不変データ `規則` と同じ名前を対象にする";
const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数: &str = "{\n    fn 変える(&mut self) {}\n}\n";

fn 違反が1件だけあり説明が含む(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>, 含む語: &str) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn 再公開を1段たどって取り込んだ実装は定義に属する() {
    for 再公開 in ["pub use x::規則;\n", "pub use x::{他, 規則};\n", "pub use self::x::*;\n"] {
        違反が1件だけあり説明が含む(
            vec![
                ソース("crates/a/src/lib.rs", 再公開),
                ソース("crates/a/src/x.rs", 定義),
                ソース("crates/a/src/y.rs", &format!("use crate::規則;\nimpl 規則 {可変の関数}")),
            ],
            自己変更の違反,
        );
    }
}

#[test]
fn globで定義のモジュールを取り込んだ実装は定義に属する() {
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/x/tests.rs", &format!("use super::*;\nimpl 規則 {可変の関数}"))], 自己変更の違反);
}

#[test]
fn 丸括弧で囲んだ型のトレイトの実装は定義に属する() {
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &format!("{定義}trait 変更 {{\n    fn 変える(&mut self);\n}}\nimpl 変更 for (規則) {可変の関数}"))], 自己変更の違反);
}

#[test]
fn 同じクレートで取り込みを解けないglobの実装は違反になる() {
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/z.rs", &format!("use super::*;\nimpl 規則 {可変の関数}"))], 結び付けられない違反);
}

#[test]
fn 定義へ結び付けられないパスの実装は違反になる() {
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/y.rs", &format!("impl crate::無い::規則 {可変の関数}"))], 結び付けられない違反);
}

#[test]
fn useの別名とtypeの別名を通した実装は違反になる() {
    for 別名の書き方 in ["use crate::x::規則 as 別名;\n", "use crate::x::規則;\ntype 別名 = 規則;\n", "use crate::x::規則;\npub type 別名 = crate::x::規則;\n"] {
        違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/y.rs", &format!("{別名の書き方}impl 別名 {可変の関数}"))], 結び付けられない違反);
    }
}

#[test]
fn 結び付けられなくても自己変更の根拠を持たない実装と別のクレートの取り込みの無い実装は違反にならない() {
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/y.rs", "impl crate::無い::規則 {\n    fn 読む(&self) {}\n}\n"),
        ソース("crates/b/src/y.rs", &format!("impl 規則 {可変の関数}")),
    ];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
