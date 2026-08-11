//! 取り込みの名前を奪う宣言の検出の検査。
//! 検出の綴りを1つの並びで書くとこの検査の対象が自分自身になるため、試験のファイルでも分けて連結する。

use std::path::Path;

use super::builtin_include_bytes::ファイル1つを検査する;

fn 違反の数(内容: &str) -> usize {
    ファイル1つを検査する(Path::new("crates/例/src/例.rs"), 内容).len()
}

#[test]
fn 同じ名前のマクロの定義を違反にする() {
    let 原文 = concat!("macro_rules! include_", "bytes {\n    () => {};\n}");
    assert_eq!(違反の数(原文), 1);
}

#[test]
fn 同じ名前の取り込みを違反にする() {
    let 原文 = concat!("use 別の場所::include_", "bytes;");
    assert_eq!(違反の数(原文), 1);
}

#[test]
fn コメントや文字列での言及は違反にしない() {
    let 原文 = concat!(
        "//! macro_rules! include_",
        "bytes を定義してはならない\nlet 語 = \"use 別の場所::include_",
        "bytes;\";"
    );
    assert_eq!(違反の数(原文), 0);
}

#[test]
fn 組み込みの呼び出しは違反にしない() {
    let 原文 = "const A: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/a.spv\"));";
    assert_eq!(違反の数(原文), 0);
}
