//! Graphiteの生成物の中の実装に、設計解釈マーカーの実装の置き場の警告を出さないことの試験。
//! 見出しと生成元の宣言が結び付いた生成物の中の実装だけを外し、手書きのファイルの実装と、見出しを偽装した手書きのファイルの実装には警告を出し続けることを固定する。
//! 材料はIssue #187の計測で置いた置き場の宣言の宣言と、生成物がノードの値の型へ書く実装の形である。

use std::path::PathBuf;

use super::scan_entry::原文一覧を検査する;

const 宣言のパス: &str = "crates/blitz_esca/src/置き場の宣言.rs";
const 生成物のパス: &str = "crates/blitz_esca/src/generated/置き場の網.rs";

const 宣言の原文: &str = "use blitz_design::M不変データ;\n\npub struct 仮の地点 {\n    標高: f32,\n}\n\nimpl M不変データ for 仮の地点 {}\n\n#[allow(non_snake_case)]\npub mod 置き場の網 {\n    include!(\"generated/置き場の網.rs\");\n}\n\n#[rustfmt::skip]\ngraphite::dynamic_graph_schema! {\n    generated = \"generated/置き場の網.rs\";\n    schema 置き場の網 {\n        node 仮の地点;\n    }\n}\n";

const 生成物の本文: &str = "#[allow(unused_imports)]\nuse super::*;\nimpl 置き場の網Insertable for super::仮の地点 {}\n";

fn 生成物の原文(一行目: &str) -> String {
    format!("{一行目}\n// 生成元: src/置き場の宣言.rs:15\n// 再生成: 案内\n\n{生成物の本文}")
}

fn 警告の出たパス一覧(原文一覧: Vec<(&str, String)>) -> Vec<PathBuf> {
    let 報告 = 原文一覧を検査する(原文一覧.into_iter().map(|(パス, 原文)| (PathBuf::from(パス), 原文)).collect());
    報告.警告一覧().iter().map(|警告| 警告.パス.clone()).collect()
}

#[test]
fn 生成物の中の実装には置き場の警告を出さない() {
    let 見出し = "// このファイルは Graphite が生成したため手編集しないこと。";
    let 原文一覧 = vec![(宣言のパス, 宣言の原文.to_string()), (生成物のパス, 生成物の原文(見出し))];
    assert!(警告の出たパス一覧(原文一覧.clone()).is_empty());
    let 報告 = 原文一覧を検査する(原文一覧.into_iter().map(|(パス, 原文)| (PathBuf::from(パス), 原文)).collect());
    let 外した一覧: Vec<&PathBuf> = 報告.対象外にした生成物一覧().iter().flat_map(|外した生成物| 外した生成物.パス一覧()).collect();
    assert_eq!(外した一覧, vec![&PathBuf::from(生成物のパス)], "外した生成物を名前つきで報告する");
}

#[test]
fn 見出しの無い同じ中身のファイルには置き場の警告を出す() {
    let 原文一覧 = vec![(宣言のパス, 宣言の原文.to_string()), (生成物のパス, 生成物の原文("// 手書きのファイル"))];
    assert_eq!(警告の出たパス一覧(原文一覧), vec![PathBuf::from(生成物のパス)]);
}

#[test]
fn 見出しを偽装した手書きのファイルには置き場の警告を出す() {
    let 見出し = "// このファイルは Graphite が生成したため手編集しないこと。";
    let 偽装のパス = "crates/blitz_esca/src/generated/偽装.rs";
    let 原文一覧 = vec![(宣言のパス, 宣言の原文.to_string()), (偽装のパス, 生成物の原文(見出し))];
    assert_eq!(警告の出たパス一覧(原文一覧), vec![PathBuf::from(偽装のパス)], "生成元が生成先に宣言していないファイルは生成物と認めない");
}
