//! 検収が見つけた、実装自身の型引数と同じ名前を修飾したパスで境界に書いた実装の反例の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 型引数が隠すのは修飾していない裸の名前だけであり、`self::規則` はモジュールの型 `規則` を指す。型引数と同じ名前を修飾した現れまで除いていたとき、`where` 句と型引数の並びの境界に書いた `self::規則` と、同じ形の全称の実装の値で受ける `self` が検査から落ちた。
//! 型引数の名前を変えた対照(`impl<T, 型> 変更<型> for Vec<T>`)が違反のまま残ることも固定する。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 型引数に隠れた名前を修飾して境界の句に書いた実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更<X> {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T, 規則> 変更<規則> for Vec<T>\nwhere\n    T: DerefMut<Target = self::規則>,\n{\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";
const 型引数に隠れた名前を修飾して型引数の並びの境界に書いた実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更<X> {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: DerefMut<Target = self::規則>, 規則> 変更<規則> for Vec<T> {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";
const 型引数に隠れた名前を修飾して境界の句に書き値で受ける全称の実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更<X> {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T, 規則> 変更<規則> for T\nwhere\n    T: DerefMut<Target = self::規則>,\n{\n    fn 変える(mut self) {\n        self.0 = 1;\n    }\n}\n";
const 型引数の名前を変えた対照の実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更<X> {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T, 型> 変更<型> for Vec<T>\nwhere\n    T: DerefMut<Target = self::規則>,\n{\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

#[test]
fn 型引数と同じ名前を修飾したパスで境界に書いた実装を違反にする() {
    for (本文, 場所) in [
        (型引数に隠れた名前を修飾して境界の句に書いた実装, "`where` 句に名前 `規則` を識別子として含み"),
        (型引数に隠れた名前を修飾して型引数の並びの境界に書いた実装, "型引数の並びの境界に名前 `規則` を識別子として含み"),
        (型引数に隠れた名前を修飾して境界の句に書き値で受ける全称の実装, "`where` 句に名前 `規則` を識別子として含み"),
        (型引数の名前を変えた対照の実装, "`where` 句に名前 `規則` を識別子として含み"),
    ] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(
            説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません") && 説明.contains(場所)),
            "{説明一覧:?}"
        );
    }
}
