//! `impl` の見出しを本体を開く `{` まで集める読み方の試験。本体に届いた見出しの表記と構文と、本体に届かない見出しを実装の見出しと読むかどうかを確かめる。
//! 前置きの属性の中の `{` と `;` は、本体に届いたときも届かないときも括弧と項目の区切りに数えないことを固定する。

use super::{implの見出しを読む, implの見出しを集める, 実装の見出しを集めた結果};

fn 行一覧を作る<const 件数: usize>(行一覧: [&str; 件数]) -> [String; 件数] {
    行一覧.map(str::to_string)
}

#[test]
#[allow(clippy::expect_used)]
fn 定数式が閉じた行からでも本体を開く波括弧までを読む() {
    let 行一覧 = 行一覧を作る(["impl<T> 規則<T> where T: 境界<{", "1 }> {", "fn 変える(&mut self) {}", "}"]);
    let 見出し = implの見出しを読む(&行一覧, 0).expect("実装の見出しを読む");
    assert_eq!(見出し.表記(), "impl<T> 規則<T> where T: 境界<{ 1 }> {");
}

#[test]
#[allow(clippy::expect_used)]
fn unsafeの付いた実装の見出しも読む() {
    let 行一覧 = 行一覧を作る(["unsafe impl 変更可能 for 規則 {", "    fn 変える(&mut self) {}", "}"]);
    let 構文 = implの見出しを読む(&行一覧, 0).map(|見出し| 見出し.構文を取り出す()).expect("unsafe の実装を読む");
    assert_eq!(構文.トレイトの表記(), Some("変更可能"));
    assert_eq!(構文.対象.名前(), "規則");
}

#[test]
#[allow(clippy::expect_used)]
fn 属性の中の項目の区切りを数えずに実装の見出しを読む() {
    let 行一覧 = 行一覧を作る([r#"#[cfg_attr(x, doc = "a;b")] impl 型 {"#, "}"]);
    let 構文 = implの見出しを読む(&行一覧, 0).map(|見出し| 見出し.構文を取り出す()).expect("属性の付いた実装を読む");
    assert_eq!(構文.対象.名前(), "型");
}

#[test]
#[allow(clippy::expect_used)]
fn 属性の中の波括弧を本体と取り違えずに実装の見出しを読む() {
    let 行一覧 = 行一覧を作る([r##"#[doc = "{"] impl 型 {"##, "}"]);
    let 構文 = implの見出しを読む(&行一覧, 0).map(|見出し| 見出し.構文を取り出す()).expect("属性の付いた実装を読む");
    assert_eq!(構文.対象.名前(), "型");
}

#[test]
fn 属性の中に波括弧があり本体に届かない行は実装の見出しでない() {
    let 行一覧 = 行一覧を作る([r##"#[doc = "{"] impl<T: 甲<U> 型;"##]);
    assert!(matches!(implの見出しを集める(&行一覧, 0), Some(実装の見出しを集めた結果::実装の見出しでない)));
}
