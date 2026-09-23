//! 実装の見出しの構文の試験。空白を挟まない `for` と `where` の読みと、名前の中の `for` と高階の寿命の束縛を区切りと読まないことを固定する。

use super::{実装の種類, 実装の見出しの構文};

// 見出しを読んだトレイトの表記と対象の型の表記。固有の実装ならトレイトの表記は無い。
fn 読んだ組(見出し: &str) -> Option<(Option<String>, String)> {
    let 構文 = 実装の見出しの構文::読む(見出し)?;
    let トレイト = match 構文.種類 {
        実装の種類::固有の実装 => None,
        実装の種類::トレイトの実装 { トレイトの表記 } => Some(トレイトの表記),
    };
    Some((トレイト, 構文.対象.表記().to_string()))
}

#[test]
fn 空白を挟まないforとwhereを識別子の境界で読む() {
    assert_eq!(読んだ組("impl 初期化 for(規則) {"), Some((Some("初期化".to_string()), "規則".to_string())));
    assert_eq!(読んだ組("impl 初期化 for&mut 規則 {"), Some((Some("初期化".to_string()), "規則".to_string())));
    assert!(実装の見出しの構文::読む("impl 初期化 for&mut 規則 {").is_some_and(|構文| 構文.対象.可変参照か()));
    assert_eq!(読んだ組("impl<T> 初期化 for 規則<T>where T: Clone {"), Some((Some("初期化".to_string()), "規則<T>".to_string())));
}

#[test]
fn 名前の中のforと高階の寿命の束縛はトレイトの実装の区切りと読まない() {
    assert_eq!(読んだ組("impl forward {"), Some((None, "forward".to_string())));
    assert_eq!(読んだ組("impl dyn for<'a> Fn(&'a u8) {"), Some((None, "dyn for<'a> Fn(&'a u8)".to_string())));
}

#[test]
fn 参照とpinを繰り返し外した対象と属性の付いた型引数を全称の実装と読む() {
    for 見出し in [
        "impl<T> 変更 for T {",
        "impl<T> 変更 for &mut &mut T {",
        "impl<T> 変更 for Pin<&mut T> {",
        "impl<'a, T> 変更 for &'a (&mut T) {",
        "impl<#[cfg(test)] T: 境界> 変更 for T {",
    ] {
        assert!(実装の見出しの構文::読む(見出し).is_some_and(|構文| 構文.全称の実装か()), "{見出し}");
    }
    for 見出し in ["impl<T> 変更 for Vec<T> {", "impl<T> 変更 for &mut [T] {", "impl 変更 for &mut 規則 {"] {
        assert!(実装の見出しの構文::読む(見出し).is_some_and(|構文| !構文.全称の実装か()), "{見出し}");
    }
}
