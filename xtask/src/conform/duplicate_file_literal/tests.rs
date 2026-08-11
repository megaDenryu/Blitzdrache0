//! ファイル名らしい綴りの選び方と、試験の項目の除かれ方の検査。走査と集計でなく規則だけを見る。

use super::extract::拡張子を含むか;
use super::test_item_skip::{範囲の中の行か, 試験の項目の行範囲一覧};
use super::ファイル内の初出を集める;

#[test]
fn 拡張子つきの綴りを選ぶ() {
    assert!(拡張子を含むか("shaders/scene.slang"));
    assert!(拡張子を含むか("a/b.png"));
}

#[test]
fn 数の綴りを拡張子と読み違えない() {
    assert!(!拡張子を含むか("0.05"));
}

#[test]
fn 点に続く並びが長すぎるものは拡張子でない() {
    assert!(!拡張子を含むか("終わり.abcdefghijk"));
}

#[test]
fn 波括弧で閉じる試験の項目だけを除く() {
    let 原文 = "const 名: &str = \"a.png\";\n#[cfg(test)]\nmod tests {\n    const 見本: &str = \"b.png\";\n}\nconst 後: &str = \"c.png\";";
    let 初出 = ファイル内の初出を集める(原文);
    assert!(初出.contains_key("a.png"));
    assert!(!初出.contains_key("b.png"));
    assert!(初出.contains_key("c.png"), "試験の項目の後ろが打ち切られている");
}

#[test]
fn 一行で閉じる試験の宣言の後ろを打ち切らない() {
    let 原文 = "#[cfg(test)]\nmod spirv_checks;\nconst 埋め込み: &str = \"ui_vertex.spv\";";
    let 初出 = ファイル内の初出を集める(原文);
    assert_eq!(初出.get("ui_vertex.spv"), Some(&3));
}

#[test]
fn 試験の項目の中の波括弧の文字リテラルで範囲が伸びない() {
    let 原文 = "#[cfg(test)]\nfn t() {\n    let c = '{';\n}\nconst 後: &str = \"after.png\";";
    let 初出 = ファイル内の初出を集める(原文);
    assert_eq!(初出.get("after.png"), Some(&5), "試験の項目の範囲が末尾まで伸びている");
}

#[test]
fn コメントと文字列の中の属性を項目の始まりと読まない() {
    let 原文 = "// #[cfg(test)]\nconst 名: &str = \"d.png\";\nlet 語 = \"#[cfg(test)]\";\nconst 後: &str = \"e.png\";";
    assert!(試験の項目の行範囲一覧(原文).is_empty());
    let 初出 = ファイル内の初出を集める(原文);
    assert!(初出.contains_key("d.png") && 初出.contains_key("e.png"));
}

#[test]
fn 試験の項目の中の行だけが範囲に入る() {
    let 範囲一覧 = 試験の項目の行範囲一覧("#[cfg(test)]\nmod tests {\n}\nconst 名: &str = \"f.png\";");
    assert!(範囲の中の行か(&範囲一覧, 2));
    assert!(!範囲の中の行か(&範囲一覧, 4));
}
