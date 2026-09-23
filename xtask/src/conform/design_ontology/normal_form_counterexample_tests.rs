//! 読み違いを閉じる正規形の試験。予約語でない名前の生の識別子・半角空白と改行でないコードの空白・型の別名の型引数の既定値が違反になり、空白を挟まない `for` がトレイトの実装として読まれることを固定する。
//! 反例はどれも rustc 1.94.0 でコンパイルが通り、自己変更の禁止を黙って迂回していた(`use crate::x::r#規則 as 法則;`・`impl 初期化 for\t規則 {}`・`type 同じ<T = 規則> = T;`・`impl 初期化 for(規則) {}`)。

use super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 予約語でない名前の生の識別子: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\nmod x {\n    pub struct 規則(pub u8);\n}\nimpl M不変データ for x::規則 {}\nuse crate::x::r#規則 as 法則;\nimpl 変更 for 法則 {\n    fn 変える(&mut self) {\n        self.0 = 1;\n    }\n}\n";
const タブで区切った実装: &str = "pub trait M不変データ {}\npub trait 初期化 {\n    fn 初期化する(&mut self) {}\n}\nmacro_rules! 通す {\n    ($($t:tt)*) => {\n        $($t)*\n    };\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\n通す! {\n    impl 初期化 for\t規則 {}\n}\n";
const 型引数の既定値を経由した別名: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub trait 別の変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 同じ<T = 規則> = T;\nimpl 変更 for 同じ {\n    fn 変える(&mut self) {\n        self.0 = 1;\n    }\n}\npub type 箱<T = 規則> = Vec<T>;\nimpl 別の変更 for 箱 {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";

fn 正規形の違反の説明一覧(本文: &str) -> Vec<String> {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
}

#[test]
fn 予約語でない名前の生の識別子を違反にし予約語の生の識別子は違反にしない() {
    let 説明一覧 = 正規形の違反の説明一覧(予約語でない名前の生の識別子);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("生の識別子 `r#規則` は予約語でない名前に `r#` を付けている"), "{}", 説明一覧[0]);
    assert!(正規形の違反の説明一覧("pub fn 生の識別子() {\n    let r#type = 1;\n    let r#match = r#type;\n}\n").is_empty());
}

#[test]
fn 半角空白と改行でないコードの空白を違反にする() {
    let 説明一覧 = 正規形の違反の説明一覧(タブで区切った実装);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("U+0009"), "{}", 説明一覧[0]);
    for (本文, 符号位置) in [("pub struct\u{200e}甲;\n", "U+200E"), ("pub struct\u{c}甲;\n", "U+000C"), ("pub struct 甲;\rpub struct 乙;\n", "U+000D")] {
        let 説明一覧 = 正規形の違反の説明一覧(本文);
        assert!(説明一覧.iter().any(|説明| 説明.contains(符号位置)), "{符号位置}: {説明一覧:?}");
    }
}

#[test]
fn 文字列とコメントの中の空白と復帰改行は違反にしない() {
    assert!(正規形の違反の説明一覧("pub const 甲: &str = \"\t\";\n// \t\u{200e}\npub struct 乙;\r\npub struct 丙;\r\n").is_empty());
}

#[test]
fn 型の別名の型引数の既定値を違反にし定数の引数と境界の中の等式は違反にしない() {
    let 説明一覧 = 正規形の違反の説明一覧(型引数の既定値を経由した別名);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧[0].contains("型の別名 `同じ` の型引数 `T = 規則` が既定値を持つ"), "{}", 説明一覧[0]);
    assert!(説明一覧[1].contains("型の別名 `箱` の型引数 `T = 規則` が既定値を持つ"), "{}", 説明一覧[1]);
    assert!(正規形の違反の説明一覧("pub type 定数<const N: usize = 3> = [u8; N];\npub type 境界<T: Iterator<Item = u8>> = T;\n").is_empty());
}

#[test]
fn 空白を挟まないforのトレイトの実装を自己変更の禁止の検査へ届ける() {
    let 本文 = "pub trait 初期化 {\n    fn 初期化する(&mut self) {}\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\nimpl 初期化 for(規則) {}\n";
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)]);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則`")), "{説明一覧:?}");
}
