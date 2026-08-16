//! 依存節の行の読み取りの試験。担当するのは、`default-features = false`の綴りを機能一覧と読み違えないことと、
//! 1行で閉じていない宣言を宣言でない行と区別することの2つを固定することである。
//! どちらも読み違えたときに検査が黙って合格を返す形の欠陥であり、症状が出ない。

#![allow(clippy::unwrap_used)]

use super::declaration::依存節の行;
use super::ledger::項を作る;

fn 宣言を読む(行: &str) -> super::declaration::ワークスペース依存の宣言 {
    match 依存節の行::読み取る(行) {
        依存節の行::宣言(宣言) => 宣言,
        _ => panic!("宣言として読めるはずの行が読めなかった: {行}"),
    }
}

#[test]
fn 版だけの宣言は既定機能を使い機能を明示しない() {
    let 宣言 = 宣言を読む("ash = \"0.38\"");
    assert_eq!(宣言.名前(), "ash");
    assert!(宣言.台帳と食い違う点一覧(&項を作る("ash", true, &[])).is_empty());
}

#[test]
fn 既定機能を切る宣言だけを機能一覧と読み違えない() {
    let 宣言 = 宣言を読む("egui-winit = { version = \"0.31\", default-features = false }");
    assert!(宣言.台帳と食い違う点一覧(&項を作る("egui-winit", false, &[])).is_empty());
}

#[test]
fn 既定機能を切りつつ機能を明示した宣言を両方読む() {
    let 宣言 = 宣言を読む("image = { version = \"0.25\", default-features = false, features = [\"png\", \"jpeg\"] }");
    assert!(宣言.台帳と食い違う点一覧(&項を作る("image", false, &["png", "jpeg"])).is_empty());
}

#[test]
fn 機能の並び順が違えば食い違いとして述べる() {
    let 宣言 = 宣言を読む("image = { version = \"0.25\", default-features = false, features = [\"jpeg\", \"png\"] }");
    assert_eq!(宣言.台帳と食い違う点一覧(&項を作る("image", false, &["png", "jpeg"])).len(), 1);
}

#[test]
fn 機能を足したのに台帳が古ければ食い違いとして述べる() {
    let 宣言 = 宣言を読む("gltf = { version = \"1\", features = [\"extras\"] }");
    assert_eq!(宣言.台帳と食い違う点一覧(&項を作る("gltf", true, &[])).len(), 1);
}

#[test]
fn 見出しと空行と注釈は宣言でない() {
    for 行 in ["[workspace.dependencies]", "", "   ", "# 参照: README"] {
        assert!(matches!(依存節の行::読み取る(行), 依存節の行::宣言でない), "宣言でないはずの行: {行}");
    }
}

#[test]
fn 一行で閉じていない宣言は宣言でない行と区別する() {
    match 依存節の行::読み取る("gltf = { version = \"1\",") {
        依存節の行::一行で閉じていない { 名前 } => assert_eq!(名前, "gltf"),
        _ => panic!("1行で閉じていない宣言として読めなかった"),
    }
}
