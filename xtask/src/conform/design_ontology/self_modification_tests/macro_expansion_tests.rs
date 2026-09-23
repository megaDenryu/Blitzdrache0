//! 自己変更の禁止の試験のうち、マクロを通した自己変更を確かめるもの。マクロの本体の中の `impl` の外の関数・丸括弧と角括弧で囲んだ本体・開き括弧が次の行にある本体と、
//! 実装の本体の直下のマクロの呼び出しと、マクロの中の実装の引数に現れるメタ変数の型への可変参照を固定する。`macro_rules!` の中の `impl` だけを見る形は、`impl 規則 { 生やす!(); }` のように関数だけを生やすマクロを通していた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 本体のマクロの違反: &str = "の実装は、本体の直下でマクロ";

fn 説明一覧(内容: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 内容)])
}

fn 違反が1件だけあり説明が含む(内容: &str, 含む語: &str) {
    let 説明一覧 = 説明一覧(内容);
    assert_eq!(説明一覧.len(), 1, "{内容}: {説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn マクロの本体の中の実装の外の自己変更の関数は違反になる() {
    違反が1件だけあり説明が含む("macro_rules! 生やす {\n    () => {\n        fn 変える(&mut self) {}\n    };\n}\n", "macro_rules! 生やす の fn 変える");
    違反が1件だけあり説明が含む("macro_rules! 生やす {\n    () => {\n        fn 変える(対象: &mut Self) {}\n    };\n}\n", "macro_rules! 生やす の fn 変える");
    違反が1件だけあり説明が含む("#[macro_export]\nmacro_rules! 生やす {\n    ($名前:ident) => {\n        fn $名前(&mut self) {}\n    };\n}\n", "macro_rules! 生やす の fn $名前");
}

#[test]
fn 関数を生やすマクロとそれを本体の直下で呼ぶ不変データの実装はどちらも違反になる() {
    let 説明一覧 = 説明一覧("macro_rules! 生やす {\n    () => {\n        fn 変える(&mut self) {}\n    };\n}\npub struct 規則;\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    生やす!();\n}\n");
    assert_eq!(説明一覧.len(), 2, "{説明一覧:?}");
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` の") && 説明.contains("マクロ `生やす!`")), "{説明一覧:?}");
    assert!(説明一覧.iter().any(|説明| 説明.contains("macro_rules! 生やす の fn 変える")), "{説明一覧:?}");
}

#[test]
fn 本体の直下のマクロの呼び出しはどの括弧でもincludeでも違反になる() {
    for 呼び出し in ["生やす!();", "生やす! {}", "生やす![];", "crate::生やす!();", "include!(concat!(env!(\"OUT_DIR\"), \"生える\"));"] {
        違反が1件だけあり説明が含む(&format!("pub struct 規則;\nimpl M不変データ for 規則 {{}}\nimpl 規則 {{\n    {呼び出し}\n}}\n"), 本体のマクロの違反);
    }
}

#[test]
fn 丸括弧と角括弧で囲んだマクロの本体の中の実装は違反になる() {
    let 実装 = "    ($型:ident) => {\n        impl $型 {\n            pub fn 足す(&mut self) {}\n        }\n    };\n";
    違反が1件だけあり説明が含む(&format!("macro_rules! 量を定義する (\n{実装});\n"), "対象の型を決められない実装 `impl $型`");
    違反が1件だけあり説明が含む(&format!("macro_rules! 量を定義する [\n{実装}];\n"), "対象の型を決められない実装 `impl $型`");
    違反が1件だけあり説明が含む(&format!("macro_rules! 量を定義する\n{{\n{実装}}}\n"), "対象の型を決められない実装 `impl $型`");
}

#[test]
fn マクロの中の実装がメタ変数の型への可変参照を引数に取る関数は違反になる() {
    let 内容 = "macro_rules! 生やす {\n    ($型:ident) => {\n        impl $型 {\n            fn 足す(左: &mut $型) {}\n        }\n    };\n}\n";
    違反が1件だけあり説明が含む(内容, "対象の型を決められない実装 `impl $型`");
}

#[test]
fn 本体の直下の比較と関数の本体の中のマクロは違反にならない() {
    let 説明一覧 = 説明一覧("pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    const 等しくない: bool = 1 != 2;\n    fn 読む(&self) {\n        println!(\"読む\");\n    }\n}\n");
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
