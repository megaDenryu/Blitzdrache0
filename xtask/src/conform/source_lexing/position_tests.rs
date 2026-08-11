//! 断片の位置と行の数え方の検査。区間の切り分けそのものは`tests`が見る。
//! 位置と行がずれると、取り込みの引数かどうかの判定と違反の報告先が同時に狂う。

use super::{コードだけの行一覧, 文字列リテラル一覧};

fn リテラルの中身一覧(内容: &str) -> Vec<String> {
    文字列リテラル一覧(内容).into_iter().map(|断片| 断片.中身).collect()
}

#[test]
fn 複数行の生文字列の次の行の位置がずれない() {
    let 原文 = "let 語 = r\"1行目\n2行目\";\nlet 次 = \"d.png\";";
    let 一覧 = 文字列リテラル一覧(原文);
    assert_eq!(一覧[1].中身, "d.png");
    assert_eq!(一覧[1].開始行, 3);
}

#[test]
fn コードだけの行はコメントと文字列を落とす() {
    let 原文 = "// #[cfg(test)]\nlet 語 = \"#[cfg(test)]\";\n#[cfg(test)]\nmod tests;";
    let 行一覧 = コードだけの行一覧(原文);
    assert!(!行一覧[0].contains("cfg(test)"));
    assert!(!行一覧[1].contains("cfg(test)"));
    assert!(行一覧[2].contains("#[cfg(test)]"));
}

#[test]
fn 復帰改行の行末継続でも綴りが繋がる() {
    let 原文 = concat!("let 語 = \"scene.", r"\", "\r\n    slang\";");
    assert_eq!(リテラルの中身一覧(原文), vec!["scene.slang"]);
}
