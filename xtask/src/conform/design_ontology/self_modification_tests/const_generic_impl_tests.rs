//! 定数ジェネリクスの式ブロックより後ろにある実装本体の可変メソッドを検査する試験。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 定数ジェネリクスの式があっても固有の実装の可変メソッドは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "trait 境界<const N: usize> {}\n#[derive(Clone)]\npub struct 規則<T>(T);\nimpl<T: Clone> M不変データ for 規則<T> {}\nimpl<T: Clone> M規則 for 規則<T> {}\nimpl<T> 規則<T>\nwhere\n    T: 境界<{ 1 }>,\n{\n    pub fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません"));
}

#[test]
fn 定数ジェネリクスの式があってもトレイト実装の可変メソッドは違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "trait 境界<const N: usize> {}\ntrait 変更可能 { fn 変える(&mut self); }\n#[derive(Clone)]\npub struct 規則<T>(T);\nimpl<T: Clone> M不変データ for 規則<T> {}\nimpl<T: Clone> M規則 for 規則<T> {}\nimpl<T> 変更可能 for 規則<T>\nwhere\n    T: 境界<{ 1 }>,\n{\n    fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません"));
}
