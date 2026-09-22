//! 設計オントロジーの構文検査の試験のうち、`M不変データ` を実装する型の固有の `impl` とトレイトの実装に `&mut self` を持たないこと(自己変更の禁止)を確かめるもの。
//! データの役割を実装する型はすべて `M不変データ` を実装するため、役割ごとの試験もこの1つの法則の試験として置く。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_可変参照メソッドはその型の固有のimplだけを見る() {
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\npub struct 規則の台帳;\n"),
        ソース(
            "crates/a/src/y.rs",
            "use crate::x::{規則, 規則の台帳};\nimpl 規則 {\n    pub fn 変える(&mut self) {}\n}\nimpl 規則の台帳 {\n    pub fn 足す(&mut self) {}\n}\n",
        ),
    ];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `規則` は &mut self メソッドを持てません"));
}

#[test]
fn 構文解析_イベントを実装する型の固有のimplの可変参照メソッドは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 出来事;\nimpl M不変データ for 出来事 {}\nimpl Mイベント for 出来事 {}\nimpl 出来事 {\n    pub fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `出来事` は &mut self メソッドを持てません"));
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

#[test]
fn 構文解析_トレイトの実装の中の可変参照メソッドも違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 不変値 {
    値: u32,
}
impl M不変データ for 不変値 {}
pub trait 変更できる {
    fn 変更する(&mut self);
}
impl 変更できる for 不変値 {
    fn 変更する(&mut self) {
        self.値 += 1;
    }
}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `不変値` は &mut self メソッドを持てません"));
}
