//! 存在の分類のマーカー(`M値オブジェクト`・`Mエンティティ`・`Mエンティティ識別子`・`M不変エンティティ`・`M可変エンティティ`)にも実装の正規形が課され、
//! かつ純粋データ規約は `M不変データ` を上位トレイトに持つものだけに掛かる(`Mエンティティ`・`M可変エンティティ` には掛からない)ことを固定する試験。

use super::marker_form_tests::正規形でない実装の違反;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_存在の分類のマーカーの再公開したパスを経由した実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "#[derive(Clone, PartialEq)]
pub struct 変位;
impl design_alias::M値オブジェクト for 変位 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_存在の分類のマーカーの正規形の実装は違反にならず可変参照メソッドも純粋データ規約も課さない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 旅行者 {
    識別子: 旅行者の識別子,
    体力: std::cell::Cell<u32>,
}
impl Mエンティティ for 旅行者 {
    type 識別子 = 旅行者の識別子;
    fn 識別子(&self) -> &旅行者の識別子 {
        &self.識別子
    }
}
impl blitz_design::M可変エンティティ for 旅行者 {}
impl 旅行者 {
    pub fn 回復する(&mut self) {}
}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
