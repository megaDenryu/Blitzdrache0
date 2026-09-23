//! 項目の予約語の位置の走査の試験。トークン木の外では、直前の字句が項目を始めうる閉じた集合に入る現れだけを数え、型の位置の `impl`・精密な捕捉の `use<..>`・生の識別子・属性の中の語を数えないことを固定する。
//! トークン木の中では、項目を始めない前置き(`->`・`:`・`&`・`<`・`mut`・`#[..]`・`use <..>`)の後ろでも数えることと、rustfmt が整形しきれずに1行のまま残す形(長い文字列の呼び出しの中のクロージャ・式の中のコメントの後ろの宣言)を数えることを固定する。

use super::super::source_lexing::コードだけの行一覧;
use super::item_keyword_position::{項目の予約語, 項目の予約語の走査};

// 原文を先頭の行から1つの走査で読み、違反にする予約語を行の順に並べる。
fn 違反にする予約語一覧(原文: &str) -> Vec<項目の予約語> {
    let mut 走査 = 項目の予約語の走査::default();
    コードだけの行一覧(原文).iter().flat_map(|行| 走査.行の頭でない項目の予約語一覧(行)).collect()
}

#[test]
fn 行の頭の宣言と項目を始めない現れは数えない() {
    for 原文 in [
        "impl 甲 {\n}\n#[allow(unused)] unsafe impl 甲 for 乙 {}\npub(crate) use crate::甲::乙;\n#[doc(hidden)] pub type 甲 = crate::乙;\n",
        "fn 受ける(相手: impl Fn(), 借り: &impl Fn(), 可変: &mut impl Fn(), 包み: Vec<impl Fn()>) -> impl Iterator<Item = u8> {}\n",
        "fn 借りる<'a>(値: &'a impl Display) -> &'a impl Display {}\nfn 入れ子() -> impl Iterator<Item = impl Display> {}\n",
        "fn 二つ() -> Result<impl Display, impl Debug> {}\nfn 組() -> (impl Display, u8) {}\nfn 配列() -> [impl Display; 2] {}\n",
        "fn 長い(\n    甲: impl Display,\n) -> Result<impl Display, impl Debug> {\n}\n",
        "fn 捕える<'a>(値: &'a u8) -> impl Sized + use<'a> {}\nfn 空の捕捉() -> impl Iterator<Item = u8> + use<> {}\n",
        "let r#type = 1;\nlet 使い道 = 型引数の並び;\n#[cfg_attr(feature = \"a\", ts(type = \"string\"))]\n#[cfg_attr(\n    test,\n    ts(type = \"string\")\n)]\n",
    ] {
        assert!(違反にする予約語一覧(原文).is_empty(), "{原文}");
    }
}

#[test]
fn トークン木の外で行の途中の宣言を数える() {
    assert_eq!(違反にする予約語一覧("pub struct 甲; impl 甲 {}\n"), vec![項目の予約語::実装]);
    assert_eq!(違反にする予約語一覧("pub struct 甲; pub(crate) type 乙 = 甲; pub use 甲 as 丙;\n"), vec![項目の予約語::型の別名, 項目の予約語::取り込み]);
    assert_eq!(違反にする予約語一覧("fn 甲() {\n    let 値 = 1 + { impl 乙 {} 2 };\n}\n"), vec![項目の予約語::実装]);
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
        assert_eq!(違反にする予約語一覧(呼び出し).len(), 1, "{呼び出し}");
    }
    let 定義 = "macro_rules! 生やす {\n    ($($名:ident),*) => { $( impl M不変データ for $名 {} )* };\n    () => { fn 走査する() -> impl Iterator<Item = u8> {} };\n}\n";
    assert_eq!(違反にする予約語一覧(定義), vec![項目の予約語::実装, 項目の予約語::実装]);
}

#[test]
fn rustfmtが1行のまま残す呼び出しと式の中の宣言を数える() {
    let 長い文字列 = "あ".repeat(120);
    let 原文 = format!("pub fn k() {{ h(\"{長い文字列}\", || {{ impl 甲 {{ pub fn 変える(&mut self) {{}} }} }}); }}\n");
    assert_eq!(違反にする予約語一覧(&原文), vec![項目の予約語::実装]);
    let 原文 = "pub fn k() {\n    let _x = 1 + /* 注 */ { impl 甲 { pub fn 変える(&mut self) {} } 2 };\n}\npub fn k3() {\n    h(|| /* 注 */ { impl 甲 { pub fn 変える(&mut self) {} } });\n}\n";
    assert_eq!(違反にする予約語一覧(原文), vec![項目の予約語::実装, 項目の予約語::実装]);
    let 原文 = "通す!(& impl 丙 { pub fn 変える(&mut self) {} });\npub fn k() {\n    let _ = 1 + /* 注 */ { use std::fmt::Display as 表示; impl 丙 { pub fn 変える2(&mut self) {} } 2 };\n}\n";
    assert_eq!(違反にする予約語一覧(原文), vec![項目の予約語::実装, 項目の予約語::取り込み, 項目の予約語::実装]);
    let 原文 = "pub fn m() { let _v = vec![{ impl 甲 { pub fn 変える(&mut self) {} } 1 }]; }\npub fn n(x: u8) -> u8 { match x { 0 => { impl 甲 {} 1 } _ => 2 } }\n";
    assert_eq!(違反にする予約語一覧(原文), vec![項目の予約語::実装, 項目の予約語::実装]);
}
