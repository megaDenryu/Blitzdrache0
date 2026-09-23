//! 読み口の読み違いの反例の試験。識別子の文字の判定(`·` で名前を切る)と、実装の見出しの `for` の読み(`for::` と先頭の `::` の対象)と、読めない対象の実装を黙って捨てることで、自己変更の禁止を迂回していた反例が違反になることを固定する。
//! 反例はどれも rustc 1.94.0 の `-D warnings` と clippy を通り、rustfmt で整形しても形が保たれる。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 識別子の文字とforの読み違いの反例を違反にする() {
    let 全称 = "pub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\nimpl<T·x: M不変データ> 変更 for T·x {\n    fn 変える(&mut self) {}\n}\n";
    assert!(
        全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 全称)])
            .iter()
            .any(|説明| 説明.contains("対象の型を決められない実装 `impl<T·x: M不変データ> 変更 for T·x`"))
    );
    let 固有 = "pub struct 型·for(pub u8);\nimpl M不変データ for 型·for {}\nimpl 型·for {\n    pub fn 変える(&mut self) {\n        self.0 = 1;\n    }\n}\n";
    assert!(
        全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 固有)])
            .iter()
            .any(|説明| 説明.contains("M不変データ `型·for` は自分の型への可変参照"))
    );
    for 対象 in ["for::jibun::規則", "for ::jibun::規則"] {
        let 本文 = format!("extern crate self as jibun;\npub struct 規則(pub u8);\n#[rustfmt::skip]\nimpl M不変データ {対象} {{}}\nimpl 規則 {{\n    pub fn 変える(&mut self) {{\n        self.0 = 1;\n    }}\n}}\n");
        let 正規形 = 正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", &本文)]);
        assert!(
            正規形.iter().any(|説明| 説明.contains("先頭に `::` を書いている")) && 正規形.iter().any(|説明| 説明.contains("`extern crate` を宣言している")),
            "{対象}: {正規形:?}"
        );
        let 自己変更 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &本文)]);
        assert!(自己変更.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照")), "{対象}: {自己変更:?}");
    }
}
