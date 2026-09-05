//! 署名の集め方・引数の型の読み取り・索引での判定の試験。台帳の中身でなく規則だけを見る。
//! モジュールの木がどこまでを含むかの試験は、木の規則を持つ`rust_module`が持つ。

use std::path::{Path, PathBuf};

use super::index::型の定義の索引;
use super::parameter::{丸ごと受け取る型の名前, 引数へ分ける};
use super::signature::自由関数の署名一覧;
use crate::type_metrics::ファイルの観測;

fn 型名一覧(原文: &str) -> Vec<String> {
    自由関数の署名一覧(原文).into_iter().map(|署名| 署名.関数名).collect()
}

#[test]
fn 深さ0の関数だけを集める() {
    let 原文 =
        "pub fn 外側(値: &台帳) {\n    fn 内側(値: &台帳) {}\n}\nimpl 台帳 {\n    fn 方法(&self) {}\n}\nmod 中 {\n    fn 中の関数(値: &台帳) {}\n}\n";
    assert_eq!(型名一覧(原文), vec!["外側".to_string()]);
}

#[test]
fn 折り返した引数の並びを1つの署名として読む() {
    let 原文 = "pub(in crate::app) fn 組み立てる(\n    アプリ: &mut アプリ,\n    枚数: usize,\n) -> 描画入力 {\n}\n";
    let 署名一覧 = 自由関数の署名一覧(原文);
    assert_eq!(署名一覧.len(), 1);
    assert_eq!(署名一覧[0].引数一覧, vec!["アプリ: &mut アプリ".to_string(), "枚数: usize".to_string()]);
}

#[test]
fn 型引数の中の丸括弧と矢印で引数の並びを取り違えない() {
    let 署名一覧 = 自由関数の署名一覧("fn 積む<F: Fn(&台帳) -> usize>(表: &mut 台帳, 手続き: F) {}\n");
    assert_eq!(署名一覧.len(), 1);
    assert_eq!(署名一覧[0].引数一覧.len(), 2);
}

#[test]
fn 山括弧の中の読点で引数を切らない() {
    assert_eq!(引数へ分ける("表: &BTreeMap<String, usize>, 数: usize").len(), 2);
}

#[test]
fn 参照で丸ごと受け取る型だけを読み取る() {
    assert_eq!(丸ごと受け取る型の名前("台帳: &台帳"), Some("台帳".to_string()));
    assert_eq!(丸ごと受け取る型の名前("台帳: &mut 台帳"), Some("台帳".to_string()));
    assert_eq!(丸ごと受け取る型の名前("台帳: &'a 台帳"), Some("台帳".to_string()));
    assert_eq!(丸ごと受け取る型の名前("台帳: &crate::台帳"), Some("台帳".to_string()));
}

#[test]
fn 丸ごとでない受け取り方は読み取らない() {
    assert_eq!(丸ごと受け取る型の名前("一覧: &[台帳]"), None);
    assert_eq!(丸ごと受け取る型の名前("対: &(台帳, 台帳)"), None);
    assert_eq!(丸ごと受け取る型の名前("役: &dyn 書き手"), None);
    assert_eq!(丸ごと受け取る型の名前("値: 台帳"), None);
    assert_eq!(丸ごと受け取る型の名前("一覧: &Vec<台帳>"), Some("Vec".to_string()));
}

fn 索引() -> 型の定義の索引 {
    let 観測一覧 = vec![ファイルの観測::ファイルの内容から生成する(
        PathBuf::from("crates/blitz_app/src/app/mod.rs"),
        "pub struct アプリ {\n    値: usize,\n}\n",
    )];
    型の定義の索引::観測から生成する(&観測一覧)
}

#[test]
fn 木の中の自由関数だけを検出する() {
    let 原文 = "fn 選ぶ(アプリ: &mut アプリ) {}\n";
    let 検出一覧 = 索引().親の型を丸ごと受け取る自由関数を探す(Path::new("crates/blitz_app/src/app/frame/action.rs"), 原文);
    assert_eq!(検出一覧.len(), 1);
    assert_eq!(検出一覧[0].型名, "アプリ");
    assert_eq!(検出一覧[0].関数名, "選ぶ");
}

#[test]
fn 木の外の自由関数は検出しない() {
    let 原文 = "fn 選ぶ(アプリ: &mut アプリ) {}\n";
    assert!(
        索引()
            .親の型を丸ごと受け取る自由関数を探す(Path::new("crates/blitz_app/src/input/ingest.rs"), 原文)
            .is_empty()
    );
}
