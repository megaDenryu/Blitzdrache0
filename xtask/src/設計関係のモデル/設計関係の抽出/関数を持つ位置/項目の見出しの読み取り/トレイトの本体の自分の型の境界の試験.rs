//! トレイトの本体の `Self` の境界が、上位トレイトを `trait A: B` と書いても `trait A where Self: B` と書いても同じになり、
//! 上位トレイトの宣言の抽出が読む上位トレイトの一覧とも同じ並びになることを固定する回帰試験。

use super::super::super::トレイトの宣言の行::{トレイトの宣言の読み取り, トレイトの宣言を読む};
use super::{見出しから次に開く本体を読む, 開いた波括弧};
use crate::conform::設計オントロジーの規約検査::module_path::モジュールパス;
use blitz_design_verification::型引数の境界;

// 1行の原文の見出しから、トレイトの本体の型引数の境界の一覧を読む。トレイトの本体でなければ試験の誤りとして止める。
fn 本体の境界一覧(見出し: &str) -> Vec<型引数の境界> {
    let 位置のモジュール = モジュールパス::区切り一覧から組む(&["blitz_game", "試験"]);
    let Some(次) = 見出しから次に開く本体を読む(&[見出し.to_string()], 0, &位置のモジュール) else {
        panic!("トレイトの見出しを読めない: {見出し}");
    };
    let 開いた波括弧::実装かトレイトの本体(本体) = 次.本体 else {
        panic!("トレイトの本体にならない: {見出し}");
    };
    本体.型引数の境界一覧
}

// 反証: 本体の `Self` の境界へ `where Self: B` の B だけを入れると、`trait A: B` と書いたトレイトの本体で `Self` が B の境界を失う。
#[test]
fn 上位トレイトの書き方によらず自分の型の境界が同じになる() {
    let 境界に書いた = 本体の境界一覧("pub trait 口<T: 耳>: 目 + 'static where T: 鼻 {");
    let where句に書いた = 本体の境界一覧("pub trait 口<T: 耳> where Self: 目 + 'static, T: 鼻 {");
    assert_eq!(境界に書いた, where句に書いた);
    assert!(境界に書いた.contains(&型引数の境界::生成する("Self", vec!["口".to_string(), "目".to_string()])), "{境界に書いた:?}");
}

// 上位トレイトの宣言の抽出と、トレイトの本体の `Self` の境界は、同じ読み方から同じ上位トレイトの並びを得る。
#[test]
fn 上位トレイトの宣言の読み方と同じ並びになる() {
    for 見出し in ["pub trait 口: 目 + 耳 {", "pub trait 口 where Self: 目 + 耳 {"] {
        let トレイトの宣言の読み取り::読めた(宣言) = トレイトの宣言を読む(&[見出し.to_string()], 0) else {
            panic!("トレイトの宣言を読めない: {見出し}");
        };
        assert_eq!(宣言.上位トレイト一覧, vec!["目".to_string(), "耳".to_string()]);
        assert!(本体の境界一覧(見出し).contains(&型引数の境界::生成する("Self", vec!["口".to_string(), "目".to_string(), "耳".to_string()])));
    }
}
