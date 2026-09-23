//! 字句の木の門番の試験。トークン木の外では、直前の字句が項目を始めうる閉じた集合に入る現れだけを数え、型の位置の `impl`・精密な捕捉の `use<..>`・生の識別子・属性の中の語・右辺の無い関連型を違反にしないことを固定する。
//! トークン木の中では、項目を始めない前置き(`->`・`:`・`&`・`<`・`mut`・`#[..]`・`use <..>`)の後ろでも数えることと、rustfmt が整形しきれずに1行のまま残す形(長い文字列の呼び出しの中のクロージャ・式の中のコメントの後ろの宣言)を数えることを固定する。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};

// 原文1つを本番と同じ入口で読み、読み口が読まなかった予約語の違反の説明だけを行の順に並べる。
fn 読まなかった予約語の説明一覧(本文: &str) -> Vec<String> {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
        .into_iter()
        .filter(|説明| 説明.contains("検査器の読み口が読んだ宣言は"))
        .collect()
}

#[test]
fn 行の頭の宣言と項目を始めない現れは数えない() {
    for 本文 in [
        "impl 甲 {\n}\n#[allow(unused)] unsafe impl 甲 for 乙 {}\npub(crate) use crate::甲::乙;\n#[doc(hidden)] pub type 甲 = crate::乙;\n",
        "fn 受ける(相手: impl Fn(), 借り: &impl Fn(), 可変: &mut impl Fn(), 包み: Vec<impl Fn()>) -> impl Iterator<Item = u8> {}\n",
        "fn 借りる<'a>(値: &'a impl Display) -> &'a impl Display {}\nfn 入れ子() -> impl Iterator<Item = impl Display> {}\n",
        "fn 二つ() -> Result<impl Display, impl Debug> {}\nfn 組() -> (impl Display, u8) {}\nfn 配列() -> [impl Display; 2] {}\n",
        "fn 長い(\n    甲: impl Display,\n) -> Result<impl Display, impl Debug> {\n}\n",
        "fn 捕える<'a>(値: &'a u8) -> impl Sized + use<'a> {}\nfn 空の捕捉() -> impl Iterator<Item = u8> + use<> {}\n",
        "fn 生の識別子() {\n    let r#type = 1;\n    let 使い道 = 型引数の並び;\n}\n#[cfg_attr(feature = \"a\", ts(type = \"string\"))]\n#[cfg_attr(\n    test,\n    ts(type = \"string\")\n)]\npub struct 甲;\n",
        "pub trait 領域 {\n    type 状態: M状態;\n    type 出力;\n}\ntype 表 = [u8; 1 << 4];\n",
    ] {
        assert!(読まなかった予約語の説明一覧(本文).is_empty(), "{本文}");
    }
}

#[test]
fn トークン木の外で行の途中の宣言を数える() {
    assert_eq!(読まなかった予約語の説明一覧("pub struct 甲; impl 甲 {}\n").len(), 1);
    let 説明一覧 = 読まなかった予約語の説明一覧("pub struct 甲; pub(crate) type 乙 = 甲; pub use 甲 as 丙;\n");
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧.iter().any(|説明| 説明.contains("`type`")) && 説明一覧.iter().any(|説明| 説明.contains("`use`")), "{説明一覧:?}");
    assert_eq!(読まなかった予約語の説明一覧("fn 甲() {\n    let 値 = 1 + { impl 乙 {} 2 };\n}\n").len(), 1);
}

#[test]
fn トークン木の中では項目を始めない前置きの後ろでも数える() {
    for 呼び出し in [
        "通す!(-> impl 丙 { pub fn 変える(&mut self) {} });",
        "通す!(: impl 丙 { pub fn 変える(&mut self) {} });",
        "通す!(& impl 丙 { pub fn 変える(&mut self) {} });",
        "通す!(< impl 丙 { pub fn 変える(&mut self) {} });",
        "通す!(mut impl 丙 { pub fn 変える(&mut self) {} });",
        "通す!(#[impl 丙 { pub fn 変える(&mut self) {} }]);",
        "取り込む!(use <crate::x::規則> as 法則);",
        "取り込む!(\n    & impl 丙 {}\n);",
        "crate::通す! {\n    & impl 丙 {}\n}",
    ] {
        assert_eq!(読まなかった予約語の説明一覧(呼び出し).len(), 1, "{呼び出し}");
    }
    let 定義 = "macro_rules! 生やす {\n    ($($名:ident),*) => { $( impl M不変データ for $名 {} )* };\n    () => { fn 走査する() -> impl Iterator<Item = u8> {} };\n}\n";
    let 説明一覧 = 読まなかった予約語の説明一覧(定義);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧.iter().all(|説明| 説明.contains("型の位置の予約語(`-> impl Trait`)も数える")), "{説明一覧:?}");
}

#[test]
fn rustfmtが1行のまま残す呼び出しと式の中の宣言を数える() {
    let 長い文字列 = "あ".repeat(120);
    let 本文 = format!("pub fn k() {{ h(\"{長い文字列}\", || {{ impl 甲 {{ pub fn 変える(&mut self) {{}} }} }}); }}\n");
    assert_eq!(読まなかった予約語の説明一覧(&本文).len(), 1);
    let 本文 = "pub fn k() {\n    let _x = 1 + /* 注 */ { impl 甲 { pub fn 変える(&mut self) {} } 2 };\n}\npub fn k3() {\n    h(|| /* 注 */ { impl 甲 { pub fn 変える(&mut self) {} } });\n}\n";
    assert_eq!(読まなかった予約語の説明一覧(本文).len(), 2);
    let 本文 = "通す!(& impl 丙 { pub fn 変える(&mut self) {} });\npub fn k() {\n    let _ = 1 + /* 注 */ { use std::fmt::Display as 表示; impl 丙 { pub fn 変える2(&mut self) {} } 2 };\n}\n";
    assert_eq!(読まなかった予約語の説明一覧(本文).len(), 3);
    let 本文 = "pub fn m() { let _v = vec![{ impl 甲 { pub fn 変える(&mut self) {} } 1 }]; }\npub fn n(x: u8) -> u8 { match x { 0 => { impl 甲 {} 1 } _ => 2 } }\n";
    assert_eq!(読まなかった予約語の説明一覧(本文).len(), 2);
}
