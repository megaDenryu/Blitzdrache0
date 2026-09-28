//! 同じクレートの別のモジュールに同名の型があるときの、`use` 行からたどった取り込み元による定義の採り方の試験。
//! 型の同一性は定義のモジュールパス + 型名であり、`crate::`・`super::`・波括弧の群を絶対のモジュールパスへ置き換え、`as` の別名は取り込み元を求められない違反にする。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_useの波括弧の群とsuperの修飾で取り込んだ定義を採る() {
    let 甲 = ソース(
        "crates/a/src/x.rs",
        "pub struct 規則 {
    速さ: std::cell::Cell<f32>,
}
",
    );
    let 乙 = ソース(
        "crates/b/src/x.rs",
        "pub struct 規則;
",
    );
    let 丙 = ソース(
        "crates/a/src/z/w.rs",
        "use super::super::x::{他, 規則};
impl M不変データ for 規則 {}
impl M規則 for 規則 {}
",
    );
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙, 丙]);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `規則` の定義は内部可変性 `Cell` を持てません"));
}

#[test]
fn 構文解析_useの別名は取り込み元を求められない違反として報告する() {
    let 甲 = ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;
",
    );
    let 乙 = ソース(
        "crates/a/src/y.rs",
        "use crate::x::規則 as 法則;
impl M不変データ for 法則 {}
impl M規則 for 法則 {}
",
    );
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]);
    assert!(!説明一覧.is_empty());
    assert!(説明一覧.iter().all(|説明| 説明.contains("`法則` の use の別名(as)からは取り込み元を求められない(別名を付けずに取り込む)")));
}

#[test]
fn 構文解析_同じクレートの別モジュールの同名の型は定義の探索で別の型として扱う() {
    let 甲 = ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;
impl M不変データ for 規則 {}
impl M規則 for 規則 {}
",
    );
    let 乙 = ソース(
        "crates/a/src/y.rs",
        "pub struct 規則 {
    速さ: std::cell::Cell<f32>,
}
",
    );
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]).is_empty());
}
