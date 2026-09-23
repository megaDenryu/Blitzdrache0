//! 型の別名の宣言の読み口(`type_alias_scan.rs`)の試験。属性と可視性と型引数を剥がして別名と右辺を読むことと、rustfmt が折った宣言を `;` まで繋いで読むことと、読み切れない綴りを読み切れないと答えることを固定する。

use super::declaration_reading_outcome::宣言を読んだ結末;
use super::type_alias_scan::{型の別名の宣言, 型の別名の宣言を読む};

fn 原文の書き出しを読む(原文: &str) -> 宣言を読んだ結末<型の別名の宣言> {
    let 行一覧: Vec<String> = 原文.lines().map(str::to_string).collect();
    型の別名の宣言を読む(&行一覧, 0)
}

fn 読んだ組(原文: &str) -> Option<(String, String)> {
    原文の書き出しを読む(原文).読めた値().map(|宣言| (宣言.別名, 宣言.右辺))
}

#[test]
fn 属性と可視性と型引数の既定値を剥がして別名と右辺を読む() {
    assert_eq!(読んだ組("pub(crate) type 短い名前<T> = crate::a::規則<T>;"), Some(("短い名前".to_string(), "crate::a::規則<T>".to_string())));
    assert_eq!(読んだ組("type 既定値付き<T = u8> = crate::a::規則<T>;"), Some(("既定値付き".to_string(), "crate::a::規則<T>".to_string())));
    assert_eq!(読んだ組("type 関数の型 = fn(&mut 規則) -> u8;"), Some(("関数の型".to_string(), "fn(&mut 規則) -> u8".to_string())));
}

#[test]
fn rustfmtが等号の後ろで折った宣言を次の行と繋いで読む() {
    assert_eq!(読んだ組("pub type 折れた別名 =\n    crate::a::規則<T>;"), Some(("折れた別名".to_string(), "crate::a::規則<T>".to_string())));
    assert_eq!(読んだ組("type 折れた型引数<\n    T,\n> = crate::a::規則<T>;"), Some(("折れた型引数".to_string(), "crate::a::規則<T>".to_string())));
}

#[test]
fn 別名と右辺のメタ変数は読み切れずクレートの根を指す決まったパスは読める() {
    assert!(原文の書き出しを読む("pub type 別の法則 = $元;").読めなかった宣言().is_some());
    assert!(原文の書き出しを読む("type 包み = Vec<$型>;").読めなかった宣言().is_some());
    assert!(原文の書き出しを読む("type $名 = crate::a::規則;").読めなかった宣言().is_some());
    assert_eq!(読んだ組("type 法則 = $crate::a::規則;"), Some(("法則".to_string(), "$crate::a::規則".to_string())));
}

#[test]
fn 右辺の括弧の中のセミコロンで宣言を終えない() {
    assert_eq!(読んだ組("type 配列 = [規則; 2];"), Some(("配列".to_string(), "[規則; 2]".to_string())));
}

#[test]
fn 右辺を持たない関連型の宣言と別の予約語は別名の宣言でない() {
    assert!(原文の書き出しを読む("typedef 何か = 別の何か;").読めなかった宣言().is_none());
    assert!(読んだ組("typedef 何か = 別の何か;").is_none());
    assert!(読んだ組("type 状態: M状態 + PartialEq;").is_none());
    assert!(読んだ組("type 境界の等式: 甲<X = Y>;").is_none());
}

#[test]
fn 読み切れない綴りは読み切れないと答える() {
    assert!(原文の書き出しを読む("type 途中まで<T>").読めなかった宣言().is_some());
    assert!(原文の書き出しを読む("type 閉じない<T = 規則;").読めなかった宣言().is_some());
    assert!(原文の書き出しを読む("type 空の右辺 = ;").読めなかった宣言().is_some());
}
