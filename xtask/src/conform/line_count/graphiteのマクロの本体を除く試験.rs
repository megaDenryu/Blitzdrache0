//! 行数の検査が、利用者が手で書くGraphiteのマクロの本体の行を数えから除き、マクロの外の行に1ファイル100行の原則を当てることを固定する試験。

use std::path::Path;

use super::super::line_count_allowance::上限行数;
use super::行数の上限超過を検査する;

/// マクロの外に `外の行数` 行、schemaの本体に `本体の行数` 行のコードを持つRustの原文。開きと閉じの区切り記号の行はマクロの外に数える。
fn graphiteの宣言を含む原文(外の行数: usize, 本体の行数: usize) -> String {
    let mut 原文 = String::from("graphite::dynamic_graph_schema! {\n");
    原文.push_str(&"    node 地点;\n".repeat(本体の行数));
    原文.push_str("}\n");
    原文.push_str(&"fn 手書き() {}\n".repeat(外の行数.saturating_sub(2)));
    原文
}

#[test]
fn graphiteのマクロの本体の行は数えずマクロの外の行が百行以内なら違反にしない() {
    let 原文 = graphiteの宣言を含む原文(上限行数, 300);
    assert!(行数の上限超過を検査する(Path::new("crates/x/src/行数の試験の宣言.rs"), &原文).is_empty());
}

#[test]
fn graphiteのマクロの外の行が百行を超えたら違反にし除いた行数を添える() {
    let 原文 = graphiteの宣言を含む原文(上限行数 + 1, 300);
    let 違反一覧 = 行数の上限超過を検査する(Path::new("crates/x/src/行数の試験の宣言.rs"), &原文);
    assert_eq!(違反一覧.len(), 1);
    assert!(違反一覧[0].説明.contains("コードの行が101行"), "{}", 違反一覧[0].説明);
    assert!(違反一覧[0].説明.contains("本体の300行を除いて"), "{}", 違反一覧[0].説明);
}
