//! 字句の走査が、引用符入りのコメント・逃がし記号入りの文字列・生文字列を取り違えないことの固定。
//! ここが崩れると、綴りの契約とファイル名らしい綴りの重複の2つの検査が同時に反証力を失う。

use super::{コードだけの行一覧, 字句の区分, 字句へ分ける, 文字列リテラル一覧};

fn リテラルの中身一覧(内容: &str) -> Vec<String> {
    文字列リテラル一覧(内容).into_iter().map(|断片| 断片.中身).collect()
}

#[test]
fn 引用符を含む行コメントは文字列リテラルでない() {
    let 原文 = "// 契約は \"validationエラー・警告合計件数:\" である\nlet 値 = 1;";
    assert!(リテラルの中身一覧(原文).is_empty());
}

#[test]
fn 引用符を含むブロックコメントは文字列リテラルでない() {
    assert!(リテラルの中身一覧("/* \"a/b.png\" は例 */ let 値 = 1;").is_empty());
}

#[test]
fn 入れ子のブロックコメントを最後まで飛ばす() {
    assert_eq!(
        リテラルの中身一覧("/* 外 /* 内 \"x.png\" */ まだ中 */ let 名 = \"y.png\";"),
        vec!["y.png"]
    );
}

#[test]
fn 逃がし記号の引用符で区間がずれない() {
    assert_eq!(
        リテラルの中身一覧(r#"let 語 = "彼は\"a.png\"と書いた"; let 次 = "b.png";"#),
        vec![r#"彼は"a.png"と書いた"#, "b.png"]
    );
}

#[test]
fn 逃がしで書いた綴りと素の綴りが同じ中身になる() {
    assert_eq!(
        リテラルの中身一覧(r#"let 語 = "shaders/scene\x2eslang";"#),
        リテラルの中身一覧(r#"let 語 = "shaders/scene.slang";"#)
    );
}

#[test]
fn 波括弧の文字リテラルはコードの区分でない() {
    let 原文 = "fn t() { let c = '{'; }";
    assert!(
        字句へ分ける(原文)
            .iter()
            .any(|断片| 断片.区分 == 字句の区分::文字リテラル && 断片.中身 == "{")
    );
    assert!(!コードだけの行一覧(原文)[0].contains('\''));
}

#[test]
fn 生文字列の中の引用符と逃がし記号を中身として読む() {
    assert_eq!(
        リテラルの中身一覧(r###"let 語 = r#"引用"と\の混じり"#; let 次 = "c.png";"###),
        vec![r#"引用"と\の混じり"#, "c.png"]
    );
}

#[test]
fn 生文字列と通常文字列が同じ綴りを同じ中身にする() {
    assert_eq!(
        リテラルの中身一覧(r##"let a = "x/y.png"; let b = r#"x/y.png"#;"##),
        vec!["x/y.png", "x/y.png"]
    );
}

#[test]
fn 複数行の生文字列の次の行の位置がずれない() {
    let 原文 = "let 語 = r\"1行目\n2行目\";\nlet 次 = \"d.png\";";
    let 一覧 = 文字列リテラル一覧(原文);
    assert_eq!(一覧[1].中身, "d.png");
    assert_eq!(一覧[1].開始行, 3);
}

#[test]
fn 引用符の文字リテラルで区間が開かない() {
    assert_eq!(リテラルの中身一覧("let 区切り = '\"'; let 名 = \"e.png\";"), vec!["e.png"]);
}

#[test]
fn ライフタイムを文字リテラルと読まない() {
    let 断片一覧 = 字句へ分ける("fn f<'a>(x: &'a str) -> &'a str { x }");
    assert!(断片一覧.iter().all(|断片| 断片.区分 == 字句の区分::コード));
}

#[test]
fn コードだけの行はコメントと文字列を落とす() {
    let 原文 = "// #[cfg(test)]\nlet 語 = \"#[cfg(test)]\";\n#[cfg(test)]\nmod tests;";
    let 行一覧 = コードだけの行一覧(原文);
    assert!(!行一覧[0].contains("cfg(test)"));
    assert!(!行一覧[1].contains("cfg(test)"));
    assert!(行一覧[2].contains("#[cfg(test)]"));
}
