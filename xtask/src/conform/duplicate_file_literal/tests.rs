//! ファイル名らしい綴りの選び方と、試験の項目の除かれ方と、取り込みの引数の見分け方の検査。
//! 走査と集計でなく規則だけを見る。

use super::extract::拡張子を含むか;
use super::test_item_skip::{範囲の中の行か, 試験の項目の行範囲一覧};
use super::ファイル内の出現を集める;

fn 綴り一覧(原文: &str) -> Vec<String> {
    ファイル内の出現を集める(原文).into_iter().map(|(綴り, _, _)| 綴り).collect()
}

fn 取り込みの引数か(原文: &str, 綴り: &str) -> bool {
    ファイル内の出現を集める(原文)
        .into_iter()
        .any(|(見つけた綴り, _, 引数か)| 見つけた綴り == 綴り && 引数か)
}

#[test]
fn 拡張子つきの綴りを選ぶ() {
    assert!(拡張子を含むか("shaders/scene.slang"));
    assert!(!拡張子を含むか("0.05"));
    assert!(!拡張子を含むか("終わり.abcdefghijk"));
}

#[test]
fn 波括弧で閉じる試験の項目だけを除く() {
    let 原文 = "const 名: &str = \"a.png\";\n#[cfg(test)]\nmod tests {\n    const 見本: &str = \"b.png\";\n}\nconst 後: &str = \"c.png\";";
    let 一覧 = 綴り一覧(原文);
    assert!(一覧.contains(&"a.png".to_string()));
    assert!(!一覧.contains(&"b.png".to_string()));
    assert!(一覧.contains(&"c.png".to_string()), "試験の項目の後ろが打ち切られている");
}

#[test]
fn 一行で閉じる試験の宣言の後ろを打ち切らない() {
    let 原文 = "#[cfg(test)]\nmod spirv_checks;\nconst 埋め込み: &str = \"ui_vertex.spv\";";
    assert_eq!(綴り一覧(原文), vec!["ui_vertex.spv".to_string()]);
}

#[test]
fn 試験の項目の中の波括弧の文字リテラルで範囲が伸びない() {
    let 原文 = "#[cfg(test)]\nfn t() {\n    let c = '{';\n}\nconst 後: &str = \"after.png\";";
    assert_eq!(綴り一覧(原文), vec!["after.png".to_string()], "試験の項目の範囲が末尾まで伸びている");
}

#[test]
fn コメントと文字列の中の属性を項目の始まりと読まない() {
    let 原文 = "// #[cfg(test)]\nconst 名: &str = \"d.png\";\nlet 語 = \"#[cfg(test)]\";\nconst 後: &str = \"e.png\";";
    assert!(試験の項目の行範囲一覧(原文).is_empty());
    assert_eq!(綴り一覧(原文), vec!["d.png".to_string(), "e.png".to_string()]);
}

#[test]
fn 試験の項目の中の行だけが範囲に入る() {
    let 範囲一覧 = 試験の項目の行範囲一覧("#[cfg(test)]\nmod tests {\n}\nconst 名: &str = \"f.png\";");
    assert!(範囲の中の行か(&範囲一覧, 2));
    assert!(!範囲の中の行か(&範囲一覧, 4));
}

#[test]
fn 取り込みの括弧の中の綴りだけを引数とみなす() {
    let 原文 = "const A: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/a.spv\"));\nconst B: &str = \"b.spv\";";
    assert!(取り込みの引数か(原文, "a.spv"));
    assert!(!取り込みの引数か(原文, "b.spv"));
}

#[test]
fn 名前の途中で一致する別のマクロは取り込みでない() {
    let 原文 = "const A: &[u8] = not_include_bytes!(\"a.spv\");\nconst B: &[u8] = include_bytes!(\"b.spv\");";
    assert!(!取り込みの引数か(原文, "a.spv"));
    assert!(取り込みの引数か(原文, "b.spv"));
}

#[test]
fn 同じ行でも括弧の外の綴りは引数でない() {
    let 原文 = "const C: (&str, &[u8]) = (\"c.spv\", include_bytes!(concat!(env!(\"OUT_DIR\"), \"/d.spv\")));";
    assert!(!取り込みの引数か(原文, "c.spv"));
    assert!(取り込みの引数か(原文, "d.spv"));
}

#[test]
fn 同じファイルの2つ目以降の出現も残す() {
    let 原文 = "const A: &[u8] = include_bytes!(\"a.spv\");\nconst B: &str = \"a.spv\";";
    let 出現一覧 = ファイル内の出現を集める(原文);
    assert_eq!(出現一覧.len(), 2, "同じファイルの2つ目の出現が落ちている");
    assert!(出現一覧.iter().any(|(_, _, 引数か)| !引数か), "取り込みでない出現が残っていない");
}
