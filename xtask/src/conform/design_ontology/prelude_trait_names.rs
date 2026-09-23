//! 取り込まずに書ける std のトレイトの名前の一覧。Rust 2021 の std の prelude にあるトレイトと、`derive` が実装を生やす std のトレイト(`Debug`・`Hash`)を置く。
//! 読み手は `use … as` の別名がこれらの名前を名乗らないことの検査(`import_alias_name_assertion.rs`)だけである。走査範囲に宣言が無いトレイトを検査しないことは、宣言を1件も引けないことがそのまま決めるため、この一覧を読まない。
//! 注意: prelude に無い名前をここへ足すと、同名の別のトレイトの実装を宣言を読まずに通すことになる。

const 取り込まずに書けるトレイトの名前の並び: &str =
    "Clone Copy Debug Default PartialEq Eq PartialOrd Ord Hash Drop From Into TryFrom TryInto AsRef AsMut Iterator IntoIterator DoubleEndedIterator ExactSizeIterator Extend FromIterator ToString ToOwned Send Sync Sized Unpin Fn FnMut FnOnce";

/// 取り込まずに書ける std のトレイトの名前か。
pub fn 取り込まずに書けるトレイトの名前か(名前: &str) -> bool {
    取り込まずに書けるトレイトの名前一覧().any(|書ける名前| 書ける名前 == 名前)
}

/// 取り込まずに書ける std のトレイトの名前を順に返す。
pub fn 取り込まずに書けるトレイトの名前一覧() -> impl Iterator<Item = &'static str> {
    取り込まずに書けるトレイトの名前の並び.split_whitespace()
}
