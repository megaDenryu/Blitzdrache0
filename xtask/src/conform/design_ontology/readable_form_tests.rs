//! 3つの読み口(`use`・`type`・`impl`)の全域性の試験。読み切れない綴りと行の頭でない項目の予約語が違反になることと、読み口の誤読で落ちていた形が自己変更の検査へ届くことを固定する。
//! 誤読で落ちていた形は4つである。入れ子の波括弧の `use` の別名、型引数の既定値を持つ `type` の右辺、行の途中に書いた `impl`・`type`・`use`、rustfmt が `=` の後ろで折った長い `type` である。

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
    assert!(説明一覧[0].contains("`impl` を行の頭でない位置に書いている"), "{}", 説明一覧[0]);
}

#[test]
fn 型の位置のimplと精密な捕捉のuseは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub fn 走査する(相手: impl Fn(), 借り: &impl Fn(), 可変: &mut impl Fn(), 包み: Vec<impl Fn()>) -> impl Iterator<Item = u8> {\n    [].into_iter()\n}\npub fn 捕える<'a>(値: &'a u8) -> impl Sized + use<'a> {\n    値\n}\n",
    )];
    assert!(正規形の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn rustfmtが等号の後ろで折った長い型の別名は違反にならず名前の閉包へ入る() {
    let 別名 = "規則を言い換えた長い別名".repeat(12);
    let 原文 = format!("pub struct 規則;\nimpl M不変データ for 規則 {{}}\nimpl M規則 for 規則 {{}}\npub type {別名} =\n    crate::a::x::規則;\nimpl {別名} {{\n    pub fn 変える(&mut self) {{}}\n}}\n");
    assert!(原文.lines().any(|行| 行.chars().map(|文字| if 文字.is_ascii() { 1 } else { 2 }).sum::<usize>() > 250));
    assert!(正規形の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &原文)]).is_empty());
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &原文)]);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}

#[test]
fn 閉じない群のuseと読み切れないtypeを違反にする() {
    let ソース一覧 = vec![ソース("crates/a/src/x.rs", "use crate::a::{b, c;\ntype 途中まで<T>\n")];
    let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧[0].contains("波括弧の群が同じ文の中で閉じていない"), "{}", 説明一覧[0]);
    assert!(説明一覧[1].contains("`type` の宣言がファイルの最後まで `;` に届かない"), "{}", 説明一覧[1]);
}

#[test]
fn 波括弧の無いuseでもセミコロンに届かなければ違反にする() {
    for (原文, 理由) in [
        ("pub use crate::a::b\n", "`use` の文がファイルの最後まで `;` で終わっていない"),
        ("use crate::a::b\npub struct 甲;\n", "展開した項目が1つのパスと別名に読めない"),
    ] {
        let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 原文)]);
        assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
        assert!(説明一覧[0].contains(理由), "{}", 説明一覧[0]);
    }
}

#[test]
fn 右辺の括弧の中にセミコロンを持つ型の別名が包んだ名前の閉包へ入る() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\npub trait ローカル {\n    fn 読む(&self) {}\n}\ntype 配列 = [規則; 2];\nimpl ローカル for 配列 {\n    fn 変える(&mut self) {}\n}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}

#[test]
fn 読み切れる綴りは違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "use crate::{x::{規則, 他}, y::*};\ntype 別名<T = u8> = Vec<T>;\ntype 関連の宣言 = fn(&mut u8) -> u8;\nimpl 規則 {\n    pub fn 走査する(&self) -> impl Iterator<Item = u8> {\n        [].into_iter()\n    }\n}\n",
    )];
    assert!(正規形の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
