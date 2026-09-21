//! 論理のモジュール構造と物理のモジュール構造の一致の検証の試験。日本語のモジュールの正規形を受理し、別名付けと置き場の食い違いを受理しないことを固定する。

use std::path::Path;

use super::module_structure_assertion::モジュール構造の一致検査;

fn 説明一覧(宣言元: &str, 原文: &str) -> Vec<String> {
    モジュール構造の一致検査::生成する(Path::new(宣言元).to_path_buf(), 原文).違反一覧().into_iter().map(|違反| 違反.説明).collect()
}

#[test]
fn 一致_lib_rsの日本語モジュールの正規形を受理する() {
    assert!(説明一覧("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 接触;\n").is_empty());
}

#[test]
fn 一致_日本語ディレクトリの下のモジュールの正規形を受理する() {
    assert!(説明一覧("crates/a/src/接触.rs", "#[path = \"接触/シーン.rs\"]\nmod シーン;\n").is_empty());
}

#[test]
fn 一致_ディレクトリを束ねるmod_rsを指す形を受理する() {
    assert!(説明一覧("crates/a/src/lib.rs", "#[path = \"接触/mod.rs\"]\nmod 接触;\n").is_empty());
}

#[test]
fn 一致_可視性が付く形を受理する() {
    assert!(説明一覧("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\npub(crate) mod 接触;\n").is_empty());
}

#[test]
fn 一致_path属性の無い宣言を受理する() {
    assert!(説明一覧("crates/a/src/x.rs", "mod y;\npub mod z;\nmod 内 {\n    pub struct 位置;\n}\n").is_empty());
}

#[test]
fn 不一致_モジュール名が物理のファイル名と違う宣言を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 衝突;\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("論理は `a::衝突`、物理は `a::接触`"));
    assert!(説明一覧[0].contains("正規形の #[path] は \"衝突.rs\""));
}

#[test]
fn 不一致_前置きが宣言元の置き場と違う宣言を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/x.rs", "#[path = \"y/接触.rs\"]\nmod 接触;\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("論理は `a::x::接触`、物理は `a::y::接触`"));
    assert!(説明一覧[0].contains("正規形の #[path] は \"x/接触.rs\""));
}

#[test]
fn 不一致_modの宣言が続かないpath属性を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\npub struct 位置;\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("#[path] の属性に mod の宣言が続きません"));
}

#[test]
fn 不一致_波括弧の中の宣言に付いたpath属性を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/lib.rs", "#[path = \"接触.rs\"]\nmod 接触 {\n    pub struct 位置;\n}\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("波括弧付き"));
}

#[test]
fn 不一致_表記を読めないpath属性を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/lib.rs", "#[path =]\nmod 接触;\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("属性の行に文字列がありません"));
}

#[test]
fn 不一致_波括弧の中に書いた別のファイルを指す宣言を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/x.rs", "mod 外 {\n    mod 子;\n}\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("波括弧の内側に置かれたまま別のファイルを指しています"));
}

#[test]
fn 不一致_関数の中に書いた別のファイルを指す宣言を違反にする() {
    let 説明一覧 = 説明一覧("crates/a/src/x.rs", "fn 組み立てる() {\n    mod 子;\n}\n");
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("波括弧の内側に置かれたまま別のファイルを指しています"));
}

#[test]
fn 一致_波括弧の中に書いた波括弧付きの宣言を受理する() {
    assert!(説明一覧("crates/a/src/x.rs", "mod 外 {\n    mod 内 {\n        pub struct 位置;\n    }\n}\n").is_empty());
}

#[test]
fn 一致_見出しコメントの中の波括弧を深さに数えない() {
    assert!(説明一覧("crates/a/src/x.rs", "//! 正規形は `mod 名前 {\nmod y;\n").is_empty());
}

#[test]
fn 一致_文字列リテラルの中のmodの宣言を読まない() {
    assert!(説明一覧("crates/a/src/x.rs", "mod 外 {\n    const 見本: &str = \"mod 子;\";\n}\n").is_empty());
}
