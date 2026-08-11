//! 行からファイル名らしいリテラルを取り出す規則の検査。走査と集計でなく、取り出しの規則だけを見る。

use super::extract::拡張子つきのリテラルを取り出す;

#[test]
fn 拡張子つきのリテラルを見つける() {
    assert_eq!(
        拡張子つきのリテラルを取り出す(r#"    const 監視先: &str = "shaders/scene.slang";"#),
        vec!["shaders/scene.slang".to_string()]
    );
}

#[test]
fn 同じ行の複数のリテラルをすべて取り出す() {
    assert_eq!(
        拡張子つきのリテラルを取り出す(r#"    let 対 = ["a/b.png", "c/d.ktx2"];"#),
        vec!["a/b.png".to_string(), "c/d.ktx2".to_string()]
    );
}

#[test]
fn 数の綴りを拡張子と読み違えない() {
    assert!(拡張子つきのリテラルを取り出す(r#"    let 閾値 = "0.05";"#).is_empty());
}

#[test]
fn 拡張子を持たないリテラルは取り出さない() {
    assert!(拡張子つきのリテラルを取り出す(r#"    println!("描画発行内訳:");"#).is_empty());
}

#[test]
fn 点に続く並びが長すぎるものは拡張子でない() {
    assert!(拡張子つきのリテラルを取り出す(r#"    let 文 = "終わり.abcdefghijk";"#).is_empty());
}

#[test]
fn コメントの中のファイル名は文字列リテラルでないため取り出さない() {
    assert!(拡張子つきのリテラルを取り出す("//! 参照: shaders/scene.slang").is_empty());
}
