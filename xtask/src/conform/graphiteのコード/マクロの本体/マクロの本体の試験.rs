//! Graphiteのマクロの本体の行として数えから除く行の範囲を固定する試験。
//! 除くのは開きと閉じの区切り記号の行に挟まれた行だけであり、マクロの外の手書きのRustと、Graphiteでないマクロの本体は除かない。

use super::Graphiteのマクロの本体の行;

fn 本体の行番号一覧(原文: &str) -> Vec<usize> {
    let 本体 = Graphiteのマクロの本体の行::原文から読む(原文);
    (1..=原文.lines().count()).filter(|行番号| 本体.含むか(*行番号)).collect()
}

#[test]
fn 名前で見分けるマクロの本体の行を除く() {
    let 原文 = "use graphite;\n#[rustfmt::skip]\ngraphite::dynamic_graph_schema! {\n    schema 網 {\n        node 地点;\n    }\n}\nfn 手書き() {}\n";
    assert_eq!(本体の行番号一覧(原文), vec![4, 5, 6]);
    let 原文 = "let g = graphite::graph!(網 {\n    甲 -> 乙;\n});\nstatic_graph_schema! {\n    x\n}\n";
    assert_eq!(本体の行番号一覧(原文), vec![2, 5]);
}

#[test]
fn 生成先の項を持つ最も内側のマクロをinstanceのマクロとして除く() {
    let 原文 = "外側! {\n    fn 手書き() {}\n    組織! {\n        generated = \"generated/開発チーム.rs\";\n        graph 開発チーム;\n    }\n}\n";
    assert_eq!(本体の行番号一覧(原文), vec![4, 5]);
}

#[test]
fn graphiteでないマクロと文字列とコメントの中の括弧は数えから除かない() {
    let 原文 = "let 並び = vec![\n    1,\n];\nmy_graph! {\n    甲\n}\n// graph! {\nlet 文字列 = \"graph! {\";\nfn 何か() {\n}\n";
    assert!(本体の行番号一覧(原文).is_empty());
}

#[test]
fn 一行で閉じる呼び出しは除く行を持たない() {
    assert!(本体の行番号一覧("let g = graph!(網 { 甲 -> 乙; });\n").is_empty());
}
