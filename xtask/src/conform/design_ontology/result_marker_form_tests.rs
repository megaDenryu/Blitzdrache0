//! `M結果` が正規形の検査の対象に入り、かつ純粋データ規約・`&mut self` の禁止・定義の解決の対象には入らないことを固定する試験(2026-09-20 のオーナー裁定。PR #169)。
//! `M結果` は上位トレイトを持たない。標準の `Result` への実装は `crates` 配下に定義が無いため、定義をたどる検査へ入れると「定義が見つからない」の違反になる。

use super::marker_form_tests::正規形でない実装の違反;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_標準のresultへのm結果の実装は定義の解決の違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/blitz_design/src/marker.rs",
        "pub trait M結果 {}
impl<T, E> M結果 for Result<T, E> {}
pub enum 遷移の帰結<'a> {
    成功(&'a u32),
    失敗,
}
impl M結果 for 遷移の帰結<'_> {}
impl 遷移の帰結<'_> {
    pub fn 失敗にする(&mut self) {}
}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_m結果の再公開したパスを経由した実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub enum 遷移の帰結 {
    成功,
    失敗,
}
impl design_alias::M結果 for 遷移の帰結 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}
