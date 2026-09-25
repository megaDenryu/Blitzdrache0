//! 利用者が手で書くGraphiteの宣言を原文から読む工程。受け取るのは原文、返すのは宣言が名指す生成物の置き場の表記と、宣言を含むかの判定である。
//!
//! Graphiteの宣言とは、`dynamic_graph_schema!`・`static_graph_schema!`・`graph!` の呼び出しと、schema名を名前にした静的グラフのinstanceの呼び出しのことである。
//! instanceの呼び出しはマクロの名前が利用者の語彙であり名前では見分けられないため、どの宣言も生成物の置き場を名指す `generated = "..."` の項を持つことで見分ける。
//! 照合はコメントを除いたコードと文字列リテラルの断片に対して行う。コメントでの言及を宣言と数えないためである。

use super::super::source_lexing::{字句の区分, 字句へ分ける};

/// 名前で見分けられるGraphiteのマクロ。instanceの呼び出しは名前が固定でないため、生成先の項で見分ける。
const 名前で見分けるマクロ一覧: [&str; 3] = ["dynamic_graph_schema!", "static_graph_schema!", "graph!"];

/// 原文の中の `generated = "..."` の項が名指す表記の一覧。表記は宣言のファイルの置き場からの相対パスである。
pub(super) fn 生成先の表記一覧(原文: &str) -> Vec<String> {
    let 断片一覧 = 字句へ分ける(原文);
    断片一覧
        .windows(2)
        .filter(|対| 対[0].区分 == 字句の区分::コード && 対[1].区分 == 字句の区分::文字列リテラル && 生成先の項の左辺で終わるか(&対[0].中身))
        .map(|対| 対[1].中身.clone())
        .collect()
}

/// 原文がGraphiteの宣言を1つ以上含むか。行数の台帳の区分が、載ったファイルの原文を確かめるために使う。
pub fn graphiteの宣言を含むか(原文: &str) -> bool {
    let マクロを呼ぶか = 字句へ分ける(原文)
        .iter()
        .filter(|断片| 断片.区分 == 字句の区分::コード)
        .any(|断片| 名前で見分けるマクロ一覧.iter().any(|マクロ| 識別子の境界から始まる出現が在るか(&断片.中身, マクロ)));
    マクロを呼ぶか || !生成先の表記一覧(原文).is_empty()
}

fn 生成先の項の左辺で終わるか(コード: &str) -> bool {
    let Some(左辺) = コード.trim_end().strip_suffix('=') else {
        return false;
    };
    左辺.trim_end().strip_suffix("generated").is_some_and(|前| !前.chars().next_back().is_some_and(識別子の文字か))
}

// `static_graph_schema!` の中の `graph_schema!` を `graph!` と数えないよう、直前が識別子の文字でない出現だけを数える。
fn 識別子の境界から始まる出現が在るか(コード: &str, 語: &str) -> bool {
    コード.match_indices(語).any(|(位置, _)| !コード[..位置].chars().next_back().is_some_and(識別子の文字か))
}

fn 識別子の文字か(文字: char) -> bool {
    文字.is_alphanumeric() || 文字 == '_'
}

#[cfg(test)]
mod tests {
    use super::{graphiteの宣言を含むか, 生成先の表記一覧};

    #[test]
    fn 生成先の項の文字列を読む() {
        let 原文 = "graphite::dynamic_graph_schema! {\n    generated = \"generated/経路網.rs\";\n    schema 経路網 { node 地点; }\n}\n";
        assert_eq!(生成先の表記一覧(原文), vec!["generated/経路網.rs".to_string()]);
    }

    #[test]
    fn コメントの中の生成先の項は読まない() {
        assert!(生成先の表記一覧("// generated = \"generated/a.rs\";\nlet 値 = \"generated/a.rs\";\n").is_empty());
        assert!(生成先の表記一覧("let pregenerated = \"a.rs\";\n").is_empty());
    }

    #[test]
    fn 三つのマクロとinstanceの呼び出しを宣言と数える() {
        assert!(graphiteの宣言を含むか("graphite::dynamic_graph_schema! { }\n"));
        assert!(graphiteの宣言を含むか("static_graph_schema! { }\n"));
        assert!(graphiteの宣言を含むか("let g = graphite::graph!(網 { });\n"));
        assert!(graphiteの宣言を含むか("組織! { generated = \"generated/開発チーム.rs\"; graph 開発チーム; }\n"));
    }

    #[test]
    fn コメントでの言及と別の名前のマクロは宣言と数えない() {
        assert!(!graphiteの宣言を含むか("// graph! と dynamic_graph_schema! をここで使う\nfn 何か() {}\n"));
        assert!(!graphiteの宣言を含むか("my_graph!(a);\nsubgraph!(b);\n"));
    }
}
