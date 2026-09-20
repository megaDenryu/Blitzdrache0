//! 設計オントロジーの構文検査の試験のうち、`M不変データ` を実装する型の固有の `impl` に `&mut self` を持たないこと(自己変更の禁止)を確かめるもの。
//! データの役割を実装する型はすべて `M不変データ` を実装するため、役割ごとの試験もこの1つの法則の試験として置く。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_可変参照メソッドはその型の固有のimplだけを見る() {
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\npub struct 規則の台帳;\n"),
        ソース("crates/a/src/y.rs", "impl 規則 {\n    pub fn 変える(&mut self) {}\n}\nimpl 規則の台帳 {\n    pub fn 足す(&mut self) {}\n}\n"),
    ];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `規則` は &mut self メソッドを持てません"));
}

#[test]
fn 構文解析_設定の固有のimplの可変参照メソッドは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 設定;\nimpl M不変データ for 設定 {}\nimpl M設定 for 設定 {}\nimpl 設定 {\n    pub fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `設定` は &mut self メソッドを持てません"));
}

#[test]
fn 構文解析_不変データだけを実装する型の固有のimplの可変参照メソッドは違反になる() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "pub struct 変位;\nimpl M不変データ for 変位 {}\nimpl 変位 {\n    pub fn 足す(&mut self) {}\n}\n")];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `変位` は &mut self メソッドを持てません"));
}

#[test]
fn 構文解析_状態を実装する型の固有のimplの可変参照メソッドは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 位置;\nimpl M不変データ for 位置 {}\nimpl M状態 for 位置 {}\nimpl 位置 {\n    pub fn 動かす(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `位置` は &mut self メソッドを持てません"));
}
