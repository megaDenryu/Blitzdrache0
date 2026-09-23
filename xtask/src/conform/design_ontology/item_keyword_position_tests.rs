//! 項目の予約語の位置の走査の試験。行の頭の宣言と、項目を始めない現れ(型の位置の `impl`・精密な捕捉の `use<..>`・生の識別子・属性の中の語)を数えないことと、
//! マクロの腕や1行に並べた宣言のように行の途中に書いた `impl`・`type`・`use` を数えることを固定する。

use super::item_keyword_position::{項目の予約語, 項目の予約語の走査};

fn 行の頭でない一覧(行: &str) -> Vec<項目の予約語> {
    項目の予約語の走査::default().行の頭でない項目の予約語一覧(行)
}

#[test]
fn 行の頭の宣言と項目を始めない現れは数えない() {
    for 行 in [
        "impl 甲 {",
        "#[allow(unused)] unsafe impl 甲 for 乙 {",
        "pub(crate) use crate::甲::乙;",
        "#[doc(hidden)] pub type 甲 = crate::乙;",
        "    fn 走査する(&self) -> impl Iterator<Item = u8> {",
        "fn 受ける(相手: impl Fn(), 借り: &impl Fn(), 可変: &mut impl Fn(), 包み: Vec<impl Fn()>) {}",
        "fn 捕える<'a>(値: &'a u8) -> impl Sized + use<'a> {",
        "    let r#type = 1;",
        "#[cfg_attr(feature = , ts(type = ))]",
        "    let 使い道 = 型引数の並び;",
    ] {
        assert!(行の頭でない一覧(行).is_empty(), "{行}");
    }
}

#[test]
fn 行をまたいだ属性の中の語は数えない() {
    let mut 走査 = 項目の予約語の走査::default();
    for 行 in ["#[cfg_attr(", "    feature = ,", "    ts(type = )", ")]", "pub struct 甲;"] {
        assert!(走査.行の頭でない項目の予約語一覧(行).is_empty(), "{行}");
    }
}

#[test]
fn マクロの腕と1行に並べた宣言の中の3つの予約語を数える() {
    assert_eq!(行の頭でない一覧("pub struct 甲; impl 甲 {"), vec![項目の予約語::実装]);
    assert_eq!(行の頭でない一覧("    ($($名:ident),*) => { $( impl M不変データ for $名 {} )* };"), vec![項目の予約語::実装]);
    assert_eq!(行の頭でない一覧("    ($名:ident) => { impl M不変データ for $名 {} };"), vec![項目の予約語::実装]);
    assert_eq!(行の頭でない一覧("    () => { pub type 法則 = crate::x::規則; use crate::x::規則 as 別名; };"), vec![項目の予約語::型の別名, 項目の予約語::取り込み]);
}
