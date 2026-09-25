//! `M結果` の構文の法則(結果は判別型である)を固定する試験。正本 `blitz_design` の `marker.rs` が持つ完全に修飾した `std::result::Result` への包括の実装だけが定義をたどらない特例であり、独自の `enum` は許し、独自の `struct` は名前が `Result` でも正本のファイルにあっても違反にする。
//! `M結果` は上位トレイトを持たないため、純粋データ規約は課さない。正規形の検査の対象には入る。

use super::marker_form_tests::正規形でない実装の違反;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 列挙型が必要の違反: &str = "M結果 `何かの結果` は enum 定義が必要です";

#[test]
fn 構文解析_正本の完全に修飾した標準のresultへのm結果の包括の実装だけは定義をたどらず違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/blitz_design/src/marker.rs",
        "pub trait M結果 {}
impl<T, E> M結果 for std::result::Result<T, E> {}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_正本のファイルに置いたresultという名前の独自の構造体への包括の実装は特例にならず列挙型が必要の違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/blitz_design/src/marker.rs",
        "pub struct Result<T, E> { pub 成功: T, pub 失敗: E }
impl<T, E> M結果 for Result<T, E> {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M結果 `Result` は enum 定義が必要です"));
}

#[test]
fn 構文解析_独自の列挙型へのm結果の実装は純粋データ規約と可変参照メソッドの禁止を課されず成立する() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub enum 遷移の帰結<'a> { 成功(&'a u32), 失敗, 評価不能 }
impl M結果 for 遷移の帰結<'_> {}
impl 遷移の帰結<'_> { pub fn 失敗にする(&mut self) {} }
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_独自の構造体へのm結果の実装は列挙型が必要の違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 何かの結果 { 値: u32 }
impl M結果 for 何かの結果 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(列挙型が必要の違反));
}

#[test]
fn 構文解析_結果の値を束ねた構造体へのm結果の実装は列挙型が必要の違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 何かの結果<状態, イベント> { pub 次の状態: 状態, pub 出来事一覧: Vec<イベント> }
impl<状態, イベント> M結果 for 何かの結果<状態, イベント> {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(列挙型が必要の違反));
}

#[test]
fn 構文解析_正本の外のresultという名前の構造体へのm結果の実装は特例にならず列挙型が必要の違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct Result;
impl M結果 for Result {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M結果 `Result` は enum 定義が必要です"));
}

#[test]
fn 構文解析_正本の外のresultという名前の独自の列挙型へのm結果の実装は成立する() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub enum Result { A, B }
impl M結果 for Result {}
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
