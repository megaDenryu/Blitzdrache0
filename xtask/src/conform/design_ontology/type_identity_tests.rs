//! 同名の型が複数のクレートにあるときの、定義と固有の `impl` の採り方の試験と、規則の純粋データ規約の試験。実装と同じファイルの定義を優先し、固有の `impl` は採った定義と同じクレートだけを見る。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_別クレートに同名の型があるときは実装と同じファイルの定義を採る() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 位置 {\n    pub 東: f32,\n}\nimpl MDTO for 位置 {}\n");
    let 乙 = ソース("crates/b/src/y.rs", "pub struct 位置<'a> {\n    値: &'a f32,\n}\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]).is_empty());
}

#[test]
fn 構文解析_同じファイルに定義が無く別のファイルに同名の定義が2つあれば違反にする() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 位置 {\n    pub 東: f32,\n}\n");
    let 乙 = ソース("crates/b/src/y.rs", "pub struct 位置 {\n    pub 北: f32,\n}\n");
    let 丙 = ソース("crates/c/src/z.rs", "impl MDTO for 位置 {}\n");
    let 期待 = "設計オントロジー: MDTO `位置` の同名の定義が複数のファイルにあり一意に決まらない(実装と同じファイルに定義を置く)".to_string();
    assert_eq!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙, 丙]), vec![期待]);
}

#[test]
fn 構文解析_別クレートの同名の型の固有のimplの可変参照メソッドは違反にならない() {
    let 甲 = ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;
impl M規則 for 規則 {}
",
    );
    let 乙 = ソース(
        "crates/b/src/y.rs",
        "pub struct 規則;
impl 規則 {
    pub fn 変える(&mut self) {}
}
",
    );
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]).is_empty());
}

#[test]
fn 構文解析_内部可変性を持つ規則は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則 {
    速さ: std::cell::RefCell<f32>,
}
impl M規則 for 規則 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M規則 `規則` の定義は内部可変性 `RefCell` を持てません"));
}
