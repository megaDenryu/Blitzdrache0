//! 字句の木の門番と読み口の突き合わせが、これまでの検収が見つけた反例を違反にすることの試験。反例はどれも rustc 1.94.0 でコンパイルが通り、自己変更の禁止を黙って迂回していた。
//! 反例は、英数字でない非ASCIIの文字を名前に持つマクロの呼び出し(`通す·!`)・コメントや改行を挟んだ予約語(`impl/**/丙`・`impl` の直後の改行・`use` の直後の改行)・同じ行の2つ目の実装・行の途中の `macro_rules!`・`$` の後ろの実装・マクロの引数の前置きの後ろの宣言・式の中の実装である。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 中点を名前に持つマクロの呼び出し: &str = "pub trait M不変データ {}\nmacro_rules! 通す· {\n    (-> $($t:tt)*) => {\n        $($t)*\n    };\n}\nmacro_rules! 渡す・ {\n    (-> $($t:tt)*) => {\n        $($t)*\n    };\n}\npub struct 丙(u8);\nimpl M不変データ for 丙 {}\n通す·!(-> impl 丙 { pub fn 変える(&mut self) { self.0 = 1; } });\npub struct 丁(u8);\nimpl M不変データ for 丁 {}\n渡す・!(-> impl 丁 { pub fn 変える(&mut self) { self.0 = 1; } });\nmod x {\n    pub struct 規則(pub u8);\n}\nimpl M不変データ for x::規則 {}\n通す·!(-> use crate::x::規則 as 法則;);\npub trait 変更 {\n    fn 変える(&mut self);\n}\nimpl 変更 for 法則 {\n    fn 変える(&mut self) {}\n}\n";
const コメントと改行を挟んだ予約語: &str = "pub trait M不変データ {}\nmacro_rules! 通す {\n    ($($t:tt)*) => {\n        $($t)*\n    };\n}\npub struct 丙(u8);\nimpl M不変データ for 丙 {}\n通す! {\n    impl/**/丙 {\n        pub fn 変える(&mut self) {\n            self.0 = 1;\n        }\n    }\n}\npub struct 丁(u8);\nimpl M不変データ for 丁 {}\n通す! {\n    impl\n    丁 {\n        pub fn 変える(&mut self) {\n            self.0 = 1;\n        }\n    }\n}\nmod x {\n    pub struct 規則(pub u8);\n}\nimpl M不変データ for x::規則 {}\n通す! {\n    use\n    crate::x::規則 as 法則;\n}\npub trait 変更 {\n    fn 変える(&mut self);\n}\nimpl 変更 for 法則 {\n    fn 変える(&mut self) {}\n}\n#[rustfmt::skip]\nimpl/**/M不変データ for 別名 {}\n#[rustfmt::skip]\ntype\n別名 = u8;\n";
const 同じ行の2つ目の実装: &str = "struct A; struct B;\nimpl A {} impl B {}\nfn f() { let s = c\"x\"; let b = br#\"y\"#; let _ = '\\u{1F600}'; }\n";
const 行の途中のマクロの定義: &str = "pub trait M不変データ {}\nmacro_rules! 通す {\n    ($($t:tt)*) => {\n        $($t)*\n    };\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\n通す! {\n    const _: () = (); macro_rules! 生やす {\n        ($型:ty) => {\n            impl $型 {\n                pub fn 変える(&mut self) {}\n            }\n        };\n    }\n}\n生やす!(規則);\npub fn 使う() {\n    let mut x = 規則(0);\n    x.変える();\n}\n";
const ドル記号の後ろの実装: &str = "pub trait M不変データ {}\nmacro_rules! 捨てる {\n    ($d:tt $($t:tt)*) => {\n        $($t)*\n    };\n}\npub struct 丙(pub u8);\nimpl M不変データ for 丙 {}\n捨てる!($ impl 丙 { pub fn 変える(&mut self) { self.0 = 1; } });\npub fn 使う() -> u8 { let mut x = 丙(0); x.変える(); x.0 }\n";
const マクロの引数の前置きの後ろの宣言: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 甲;\nimpl M不変データ for 甲 {}\nmacro_rules! 実装を並べる {\n    ($($名:ident: impl $トレイト:ident { $($本体:tt)* })*) => {\n        $(\n            impl $トレイト for $名 {\n                $($本体)*\n            }\n        )*\n    };\n}\n実装を並べる! {\n    甲: impl 変更 { fn 変える(&mut self) {} }\n}\npub struct 乙;\nimpl M不変データ for 乙 {}\nmacro_rules! 通す {\n    (< $($t:tt)*) => {\n        $($t)*\n    };\n    (& $($t:tt)*) => {\n        $($t)*\n    };\n    (mut $($t:tt)*) => {\n        $($t)*\n    };\n    (#[$($t:tt)*]) => {\n        $($t)*\n    };\n}\n通す!(< impl 乙 { pub fn 変える(&mut self) {} });\npub struct 丙;\nimpl M不変データ for 丙 {}\n通す!(& impl 丙 { pub fn 変える(&mut self) {} });\npub struct 丁;\nimpl M不変データ for 丁 {}\n通す!(mut impl 丁 { pub fn 変える(&mut self) {} });\npub struct 戊;\nimpl M不変データ for 戊 {}\n通す!(#[impl 戊 { pub fn 変える(&mut self) {} }]);\nmod x {\n    pub struct 規則;\n}\npub struct 己;\nimpl M不変データ for 己 {}\nmacro_rules! 取り込む {\n    (use <$パス:path> as $別名:ident) => {\n        use $パス as $別名;\n    };\n}\n取り込む!(use <crate::x::規則> as 法則);\n通す!(< type 包み = [x::規則; 2];);\n";
const マクロの本体の中のメタ変数の宣言: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\nmod x {\n    pub struct 規則;\n}\nimpl M不変データ for x::規則 {}\nmacro_rules! 取り込む {\n    ($パス:path, $別名:ident) => {\n        use $パス as $別名;\n    };\n}\n取り込む!(crate::x::規則, 法則);\nimpl 変更 for 法則 {\n    fn 変える(&mut self) {}\n}\nmacro_rules! 名付ける {\n    ($元:ty) => {\n        pub type 別の法則 = $元;\n    };\n}\n名付ける!(crate::x::規則);\nimpl 別の変更 for 別の法則 {\n    fn 変える(&mut self) {}\n}\npub trait 別の変更 {\n    fn 変える(&mut self);\n}\n";
const 式の中の実装: &str = "pub struct 甲(u8);\nconst _: () = { impl 甲 { pub fn 変える(&mut self) { self.0 = 1; } } };\npub fn f() { impl 甲 { pub fn 変える2(&mut self) {} } }\npub fn g() -> u8 { let x = { impl 甲 { pub fn 変える3(&mut self) {} } 1 }; x }\npub fn h(s: &str, _c: impl Fn()) {}\npub fn k() { h(\"ああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああ\", || { impl 甲 { pub fn 変える4(&mut self) { self.0 = 2; } } }); }\npub fn m() { let _v = vec![{ impl 甲 { pub fn 変える5(&mut self) {} } 1 }]; }\npub fn n(x: u8) -> u8 { match x { 0 => { impl 甲 { pub fn 変える6(&mut self) {} } 1 } _ => 2 } }\n";
const コメントの後ろの式の中の実装: &str = "pub struct 甲(u8);\npub fn h(_c: impl Fn()) {}\npub fn k() {\n    h(|| {\n        impl 甲 {\n            pub fn 変える(&mut self) {\n                self.0 = 2;\n            }\n        }\n    } /* 注 */);\n}\npub fn k2() {\n    let _x = 1 + /* 注 */ { impl 甲 { pub fn 変える2(&mut self) {} } 2 };\n}\npub fn k3() {\n    h(|| /* 注 */ { impl 甲 { pub fn 変える3(&mut self) {} } });\n}\n";
const マクロの引数と式の中の宣言: &str = "pub struct 丙(u8);\nmacro_rules! 通す { (& $($t:tt)*) => { $($t)* }; }\n通す!(& impl 丙 { pub fn 変える(&mut self) { self.0 = 1; } });\npub fn k() {\n    let _ = 1 + /* 注 */ { use std::fmt::Display as 表示; impl 丙 { pub fn 変える2(&mut self) {} } 2 };\n}\n";

// 原文1つを本番と同じ入口で読んだ正規形の違反の説明のうち、読み口が読まなかった予約語の違反の説明。
fn 読まなかった予約語の説明一覧(本文: &str) -> Vec<String> {
    正規形の違反の説明一覧(本文).into_iter().filter(|説明| 説明.contains("検査器の読み口が読んだ宣言は")).collect()
}

fn 正規形の違反の説明一覧(本文: &str) -> Vec<String> {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
}

#[test]
fn 中点を名前に持つマクロの引数の中の実装と取り込みを違反にする() {
    let 説明一覧 = 読まなかった予約語の説明一覧(中点を名前に持つマクロの呼び出し);
    assert_eq!(説明一覧.len(), 3, "{説明一覧:?}");
    assert_eq!(説明一覧.iter().filter(|説明| 説明.contains("`impl` が1個ある")).count(), 2, "{説明一覧:?}");
    assert!(説明一覧.iter().any(|説明| 説明.contains("`use` が1個ある")), "{説明一覧:?}");
}

#[test]
fn コメントを挟んだ実装は読み口が読み改行を挟んだ予約語は違反にする() {
    let 説明一覧 = 読まなかった予約語の説明一覧(コメントと改行を挟んだ予約語);
    assert_eq!(説明一覧.len(), 3, "{説明一覧:?}");
    for 予約語 in ["impl", "use", "type"] {
        assert!(説明一覧.iter().any(|説明| 説明.contains(&format!("`{予約語}` が1個ある"))), "{予約語}: {説明一覧:?}");
    }
    let 自己変更の説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", コメントと改行を挟んだ予約語)]);
    assert!(
        自己変更の説明一覧.iter().any(|説明| 説明.contains("M不変データ `丙` は自分の型への可変参照を受け手か引数に持つ関数を持てません")),
        "{自己変更の説明一覧:?}"
    );
}

#[test]
fn 空白を挟まないuseを違反にする() {
    assert_eq!(読まなかった予約語の説明一覧("通す! {\n    use::std::fmt;\n}\n").len(), 1);
    assert_eq!(読まなかった予約語の説明一覧("use{std::fmt, std::io};\n").len(), 1);
}

#[test]
fn 同じ行の2つ目の実装を違反にする() {
    let 説明一覧 = 読まなかった予約語の説明一覧(同じ行の2つ目の実装);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("`impl` が2個あるが、検査器の読み口が読んだ宣言は1個である"), "{}", 説明一覧[0]);
}

#[test]
fn 行の途中のマクロの定義を違反にする() {
    let 説明一覧 = 読まなかった予約語の説明一覧(行の途中のマクロの定義);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("`macro_rules!` が1個ある"), "{}", 説明一覧[0]);
}

#[test]
fn ドル記号の後ろの実装を違反にしメタ変数の改名を求める() {
    let 説明一覧 = 読まなかった予約語の説明一覧(ドル記号の後ろの実装);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("マクロのメタ変数の名前が予約語(`$type` 等)なら、名前を改める"), "{}", 説明一覧[0]);
}

#[test]
fn 前々回の反例をすべて違反にする() {
    assert_eq!(読まなかった予約語の説明一覧(マクロの引数の前置きの後ろの宣言).len(), 9);
    assert!(正規形の違反の説明一覧(マクロの引数の前置きの後ろの宣言).iter().any(|説明| 説明.contains("マクロのメタ変数がある")));
    let 説明一覧 = 正規形の違反の説明一覧(マクロの本体の中のメタ変数の宣言);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧.iter().all(|説明| 説明.contains("マクロのメタ変数がある")), "{説明一覧:?}");
}

#[test]
fn rustfmtが整形しきれない式の中の宣言をすべて違反にする() {
    assert_eq!(読まなかった予約語の説明一覧(式の中の実装).len(), 6);
    assert_eq!(読まなかった予約語の説明一覧(コメントの後ろの式の中の実装).len(), 2);
    assert_eq!(読まなかった予約語の説明一覧(マクロの引数と式の中の宣言).len(), 3);
}
