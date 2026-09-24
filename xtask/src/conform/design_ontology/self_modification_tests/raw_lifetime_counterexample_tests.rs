//! 寿命の名前を予約語の生の識別子(`'r#fn`・`'r#impl`)にした可変参照の反例の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 読み口が寿命を識別子の文字だけで読み飛ばしていたため、`&'r#fn mut self` の `r` だけを飛ばし、残りの `#fn mut self` を可変参照と読まなかった。
//! 生の識別子の正規形は予約語でない名前(`'r#a`)を別に違反にするため、反例は予約語の名前だけで作る。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 生の識別子の寿命の可変の受け手: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える<'r#fn>(&'r#fn mut self, 値: &'r#fn u8) {\n        self.0 = *値;\n    }\n}\n";
const 生の識別子の寿命で自分の型への可変参照を受ける引数: &str =
    "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える<'r#fn>(相手: &'r#fn mut Self, 値: &'r#fn u8) {\n        相手.0 = *値;\n    }\n}\n";
const 生の識別子の寿命の可変参照を包んだ引数: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える<'r#fn>(相手: Option<&'r#fn mut Self>, 値: &'r#fn u8) {\n        if let Some(中身) = 相手 {\n            中身.0 = *値;\n        }\n    }\n}\n";
const 生の識別子の寿命の可変の受け手を宣言したトレイトの実装: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<'r#fn> {\n    fn 変える(&'r#fn mut self);\n}\nimpl<'r#fn> 変更<'r#fn> for 規則 {\n    fn 変える(&'r#fn mut self) {\n        self.0 = 1;\n    }\n}\n";
const 別の予約語の生の識別子の寿命の可変の受け手: &str =
    "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える<'r#impl>(&'r#impl mut self, 値: &'r#impl u8) {\n        self.0 = *値;\n    }\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

#[test]
fn 生の識別子の寿命を付けた自分の型への可変参照を違反にする() {
    for 本文 in [
        生の識別子の寿命の可変の受け手,
        生の識別子の寿命で自分の型への可変参照を受ける引数,
        生の識別子の寿命の可変参照を包んだ引数,
        生の識別子の寿命の可変の受け手を宣言したトレイトの実装,
        別の予約語の生の識別子の寿命の可変の受け手,
    ] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(
            説明一覧
                .iter()
                .any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません") && 説明.contains("関数 `変える`")),
            "{本文}: {説明一覧:?}"
        );
    }
}
