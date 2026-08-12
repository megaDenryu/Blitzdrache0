//! 入力契約検査の試験。合格する材料と、契約を1箇所だけ崩した負の対照を突き合わせる。
//! ここが持つのは文書と幾何の契約であり、材質スロットの契約は`material_tests`が持つ。
//! 材料を組み立てて検査へ通す手順は両者が共有するため、この階層が所有して子へ貸す。

#![allow(clippy::unwrap_used)]

mod material_tests;

use super::fixture_json::{三角形のプリミティブ, 合格の指定, 文書jsonを作る};
use super::glb_fixture::{glbを書き出す, 三角形のバイナリ};
use super::result::契約検査結果;
use super::target_file::入力契約を検査するglTFのファイル;

pub(super) fn 検査する(名前: &str, 指定: &super::fixture_json::文書の指定<'_>) -> 契約検査結果 {
    let パス = glbを書き出す(名前, &文書jsonを作る(指定), &三角形のバイナリ()).unwrap();
    入力契約を検査するglTFのファイル::生成する(&パス).全項目を検査する()
}

#[test]
fn 契約を満たす三角形は違反も警告も出ない() {
    let 結果 = 検査する("合格三角形", &合格の指定());
    assert!(結果.合格か(), "指摘: {結果}");
    assert_eq!(結果.違反件数(), 0, "指摘: {結果}");
    assert_eq!(結果.警告件数(), 0, "指摘: {結果}");
    let 概要 = 結果.概要().unwrap();
    assert_eq!(概要.頂点数, 3);
    assert_eq!(概要.インデックス数, 3);
}

#[test]
fn ノード変換が単位行列でなければ違反になる() {
    let mut 指定 = 合格の指定();
    指定.ノード列 = r#"{ "mesh": 0, "translation": [3.0, 0.0, 0.0] }"#;
    let 結果 = 検査する("ノード変換あり", &指定);
    assert!(!結果.合格か(), "指摘: {結果}");
    assert_eq!(結果.違反件数(), 1, "指摘: {結果}");
}

#[test]
fn 頂点位置が無ければ違反になる() {
    let mut 指定 = 合格の指定();
    指定.プリミティブ列 = r#"{ "attributes": { "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 0 }"#;
    let 結果 = 検査する("頂点位置なし", &指定);
    assert!(!結果.合格か(), "指摘: {結果}");
}

#[test]
fn メッシュが2つあれば違反になる() {
    let mut 指定 = 合格の指定();
    let 追加 = format!(r#", {{ "name": "捨てられるメッシュ", "primitives": [ {三角形のプリミティブ} ] }}"#);
    指定.追加メッシュ列 = &追加;
    let 結果 = 検査する("メッシュ2個", &指定);
    assert!(!結果.合格か(), "指摘: {結果}");
    assert_eq!(結果.違反件数(), 2, "指摘: {結果}");
}

#[test]
fn 三角形以外の並び方は違反になる() {
    let mut 指定 = 合格の指定();
    指定.プリミティブ列 = r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 0, "mode": 1 }"#;
    let 結果 = 検査する("点の並び", &指定);
    assert!(!結果.合格か(), "指摘: {結果}");
    assert_eq!(結果.違反件数(), 1, "指摘: {結果}");
}

#[test]
fn 開けないパスは違反として報告する() {
    let 存在しないパス = std::env::temp_dir().join("blitzdrache0_contract_fixture").join("存在しない.glb");
    let 結果 = 入力契約を検査するglTFのファイル::生成する(&存在しないパス).全項目を検査する();
    assert!(!結果.合格か());
    assert!(結果.概要().is_none());
}
