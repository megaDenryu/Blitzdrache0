//! 3つの読み口(`use`・`type`・`impl`)の全域性の試験。読み切れない綴りが違反になることと、読み口の誤読で落ちていた形が自己変更の検査へ届くことを固定する。
//! 誤読で落ちていた形は3つである。入れ子の波括弧の `use` の別名、型引数の既定値を持つ `type` の右辺、行の途中から書き始めた `impl` の見出しである。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧, 正規形の説明関数を連ねた違反の説明一覧};

#[test]
fn 入れ子の波括弧のuseの別名が名前の閉包へ入る() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\n");
    let 乙 = ソース("crates/a/src/y.rs", "use crate::{x::{規則 as 法則}, z::他};\nimpl 法則 {\n    pub fn 変える(&mut self) {}\n}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}

#[test]
fn 型引数の既定値を持つ型の別名の右辺が名前の閉包へ入る() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\ntype 別名<T = u8> = 規則;\nimpl 別名 {\n    pub fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}

#[test]
fn 行の途中から書き始めたimplの見出しを読み切れない宣言として違反にする() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "pub struct 規則; impl 規則 {\n    pub fn 変える(&mut self) {}\n}\n")];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("`impl` の見出しを行の途中から書き始めている"), "{}", 説明一覧[0]);
}

#[test]
fn 閉じない群のuseと読み切れないtypeを違反にする() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "use crate::a::{b, c;\ntype 途中まで<T>\n")];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧[0].contains("波括弧の群が同じ文の中で閉じていない"), "{}", 説明一覧[0]);
    assert!(説明一覧[1].contains("`type` の宣言が同じ行の中で `;` に届かない"), "{}", 説明一覧[1]);
}

#[test]
fn 読み切れる綴りは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use crate::{x::{規則, 他}, y::*};\ntype 別名<T = u8> = Vec<T>;\ntype 関連の宣言 = fn(&mut u8) -> u8;\nimpl 規則 {\n    pub fn 走査する(&self) -> impl Iterator<Item = u8> {\n        [].into_iter()\n    }\n}\n",
    )];
    assert!(正規形の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
