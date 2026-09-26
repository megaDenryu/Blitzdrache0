//! Graphiteの生成物から外す検査が、オーナーの裁定の範囲に限られていることを固定する試験。
//! 外すのは行数・1つの宣言へ2行以上積んだ説明の注釈・参照パスの実在の3つだけであり、宣言の間のコメントだけの行は生成物にも違反とする。

use std::path::{Path, PathBuf};

use super::super::graphiteのコード::Graphiteの生成物の一覧;
use super::super::走査した原文の一覧::走査した原文の一覧;
use super::一ファイルの違反を集める;

const 生成元のパス: &str = "crates/blitz_esca/src/仮の区域網の宣言.rs";
const 生成物のパス: &str = "crates/blitz_esca/src/generated/区域網.rs";
const 手書きのパス: &str = "crates/blitz_esca/src/手書きの区域.rs";

const 宣言の原文: &str = "pub mod 区域網 {\n    include!(\"generated/区域網.rs\");\n}\n\ngraphite::dynamic_graph_schema! {\n    generated = \"generated/区域網.rs\";\n    schema 区域網 { node 仮の地点; }\n}\n";

/// 1つの宣言へ積んだ2行目の説明の注釈(本文の3行目)と宣言の間のコメントだけの行(本文の5行目)を持ち、コードの行が100行を超える本文。
fn 本文() -> String {
    let mut 本文 = String::from("pub struct 仮の地点 {\n    /// 一行目\n    /// 二行目\n    甲: u32,\n    // 生成器が書いた普通のコメント\n    乙: u32,\n}\n");
    for 番号 in 0..120 {
        本文.push_str(&format!("pub const 定数{番号}: u32 = {番号};\n"));
    }
    本文
}

fn 違反を集める(パス: &str, 原文: &str, 生成物: &Graphiteの生成物の一覧) -> Vec<(Option<usize>, String)> {
    一ファイルの違反を集める(Path::new(パス), 原文, 生成物).into_iter().map(|違反| (違反.行番号, 違反.説明)).collect()
}

#[test]
fn 生成物には宣言の間のコメントだけの行だけを違反とし説明の注釈の行数と行数は当てない() {
    let 生成物の原文 = format!("// このファイルは Graphite が生成したため手編集しないこと。\n// 生成元: src/仮の区域網の宣言.rs:5\n\n{}", 本文());
    let 一覧 = 走査した原文の一覧::生成する(vec![(PathBuf::from(生成元のパス), 宣言の原文.to_string()), (PathBuf::from(生成物のパス), 生成物の原文.clone())]);
    let 生成物 = Graphiteの生成物の一覧::原文一覧から見分ける(&一覧);
    assert!(生成物.生成物か(Path::new(生成物のパス)));
    let 違反一覧 = 違反を集める(生成物のパス, &生成物の原文, &生成物);
    assert_eq!(違反一覧.len(), 1, "{違反一覧:?}");
    assert_eq!(違反一覧[0].0, Some(8));
}

#[test]
fn 手書きのファイルには説明の注釈の行数と行数も当てる() {
    let 一覧 = 走査した原文の一覧::生成する(vec![(PathBuf::from(手書きのパス), 本文())]);
    let 生成物 = Graphiteの生成物の一覧::原文一覧から見分ける(&一覧);
    let 違反一覧 = 違反を集める(手書きのパス, &本文(), &生成物);
    let 行番号一覧: Vec<Option<usize>> = 違反一覧.iter().map(|(行番号, _)| *行番号).collect();
    assert!(行番号一覧.contains(&Some(3)), "2行目の説明の注釈を違反とする: {違反一覧:?}");
    assert!(行番号一覧.contains(&Some(5)), "コメントだけの行を違反とする: {違反一覧:?}");
    assert!(行番号一覧.contains(&None), "コードの行が100行を超える: {違反一覧:?}");
}
