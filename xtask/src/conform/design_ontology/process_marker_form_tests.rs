//! `M工程` が正規形の検査の対象に入り、かつ純粋データ規約と `&mut self` の禁止の対象には入らないことを固定する試験(PR #169 の第1回レビューの高優先度4)。
//! `M工程` は `M不変データ` を上位トレイトに持たない(工程は状態を読み書きするものであり、純粋なデータではない)。

use super::marker_form_tests::正規形でない実装の違反;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_m工程の再公開したパスを経由した実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 歩行の工程;
impl design_alias::M工程 for 歩行の工程 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_m工程の正規形の実装は純粋データ規約の対象にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 歩行の工程<'a> {
    台帳: &'a mut u32,
}
impl M工程 for 歩行の工程<'_> {}
impl 歩行の工程<'_> {
    pub fn 進める(&mut self) {}
}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
