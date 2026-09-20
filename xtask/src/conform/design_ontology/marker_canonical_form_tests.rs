//! 設計マーカーの実装の形を正規形へ固定する検査の試験のうち、別名を1度も使わずに正規形を迂回する書き方(再公開の経由・絶対パス・パスの中の空白)と、`blitz_design` の再公開を確かめるもの。
//! PR #138 の第7回レビューの必須1が挙げた3つの書き方を、そのままの形で違反にすることを固定する。

use super::marker_form_tests::正規形でない実装の違反;
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 再公開の違反: &str = "設計マーカーを再公開すると、実装の行が正規形でなくなり構文検査が実装を認識できない。`blitz_design` から直接取り込む";

#[test]
fn 構文解析_再公開したパスを経由したマーカーの実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 借り;
impl design_alias::MDTO for 借り {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_絶対パスのマーカーの実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 借り;
impl ::blitz_design::MDTO for 借り {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_パスの中に空白を置いたマーカーの実装は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 借り;
impl blitz_design :: MDTO for 借り {}
impl blitz_design::M状態 for 借り {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(正規形でない実装の違反));
}

#[test]
fn 構文解析_正規形のマーカーの実装と名前の一部にマーカー名を含むトレイトは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 位置;
impl MDTO for 位置 {}
impl blitz_design::M状態 for 位置 {}
pub trait M状態機械 {}
impl M状態機械 for 位置 {}
impl std::fmt::Display for 位置 {}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 構文解析_blitz_designの外での再公開は違反になり自身の再公開は違反にならない() {
    let ソース一覧 = vec![
        ソース(
            "crates/a/src/x.rs",
            "mod design_alias {
    pub use blitz_design::MDTO;
}
",
        ),
        ソース(
            "crates/blitz_design/src/lib.rs",
            "mod marker;
pub use marker::MDTO;
pub use blitz_design_types::設定;
",
        ),
    ];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains(再公開の違反));
}
