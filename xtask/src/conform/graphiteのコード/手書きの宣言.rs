//! 利用者が手で書くGraphiteの宣言の中の、生成物の置き場を名指す `generated = "..."` の項を原文から読む工程。
//! 受け取るのは原文、返すのは項が名指す表記と、その項の文字列が在る行番号である。
//!
//! Graphiteの宣言とは、`dynamic_graph_schema!`・`static_graph_schema!`・`graph!` の呼び出しと、schema名を名前にした静的グラフのinstanceの呼び出しのことである。
//! instanceの呼び出しはマクロの名前が利用者の語彙であり名前では見分けられないため、生成物の置き場を名指す `generated = "..."` の項を持つことで見分ける。
//! 照合はコメントを除いたコードと文字列リテラルの断片に対して行う。コメントでの言及を項と数えないためである。

use super::super::source_lexing::{字句の区分, 字句の断片, 字句へ分ける};

/// 原文の中の `generated = "..."` の項が名指す表記の一覧。表記は宣言のファイルの置き場からの相対パスである。
pub(super) fn 生成先の表記一覧(原文: &str) -> Vec<String> {
    生成先の項の文字列一覧(原文).into_iter().map(|断片| 断片.中身).collect()
}

/// 原文の中の `generated = "..."` の項の文字列が始まる行番号の一覧。行番号は1始まりである。
pub(super) fn 生成先の項の行番号一覧(原文: &str) -> Vec<usize> {
    生成先の項の文字列一覧(原文).into_iter().map(|断片| 断片.開始行).collect()
}

fn 生成先の項の文字列一覧(原文: &str) -> Vec<字句の断片> {
    let mut 項の文字列一覧 = Vec::new();
    let mut 直前の断片: Option<字句の断片> = None;
    for 断片 in 字句へ分ける(原文) {
        let 項の右辺か = 断片.区分 == 字句の区分::文字列リテラル && 直前の断片.as_ref().is_some_and(|直前| 直前.区分 == 字句の区分::コード && 生成先の項の左辺で終わるか(&直前.中身));
        直前の断片 = Some(断片);
        if 項の右辺か {
            項の文字列一覧.extend(直前の断片.take());
        }
    }
    項の文字列一覧
}

fn 生成先の項の左辺で終わるか(コード: &str) -> bool {
    let Some(左辺) = コード.trim_end().strip_suffix('=') else {
        return false;
    };
    左辺.trim_end().strip_suffix("generated").is_some_and(|前| !前.chars().next_back().is_some_and(|文字| 文字.is_alphanumeric() || 文字 == '_'))
}

#[cfg(test)]
mod tests {
    use super::{生成先の表記一覧, 生成先の項の行番号一覧};

    #[test]
    fn 生成先の項の文字列と行番号を読む() {
        let 原文 = "graphite::dynamic_graph_schema! {\n    generated = \"generated/経路網.rs\";\n    schema 経路網 { node 地点; }\n}\n";
        assert_eq!(生成先の表記一覧(原文), vec!["generated/経路網.rs".to_string()]);
        assert_eq!(生成先の項の行番号一覧(原文), vec![2]);
    }

    #[test]
    fn コメントの中の生成先の項は読まない() {
        assert!(生成先の表記一覧("// generated = \"generated/a.rs\";\nlet 値 = \"generated/a.rs\";\n").is_empty());
        assert!(生成先の表記一覧("let pregenerated = \"a.rs\";\n").is_empty());
    }
}
