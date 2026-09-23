//! 字句の木の並びの中の行の頭の `impl` と `type` の後ろの字句から、見出しの中身(`header_content.rs`)を取り出す純粋な関数。受け取るのは proc-macro2 の字句の並びと予約語の添字、返すのは見出しの中身である。
//! 字句の木の側で取り出すのは、読み口が文字列で読んだ値と答え合わせをするためである。字句の木を基準にするのは、識別子の文字・コメント・括弧の対応を rustc と同じ規則で読むためであり、構文の木は作らない。
//! 型引数の名前は各引数の先頭の識別子であり、寿命と定数の引数を除く。山括弧の外の探索は `angle_bracket_scan.rs`、型の表記の名前は `type_notation_name.rs` が持つ。

use proc_macro2::{Delimiter, TokenTree};

use super::super::line_matching::先頭の識別子;
use super::angle_bracket_scan::{最上位で止める, 最上位のカンマで分ける, 最上位の位置, 記号か, 語か, 高階の寿命の束縛か};
use super::header_content::見出しの中身;
use super::type_notation_name::{パスの最後の名前, 対象の型の名前};

/// 並びの添字の `impl` の後ろの字句から、実装の見出しの中身を取り出す。見出しは本体を開く波括弧の群か `where` までである。
pub fn 実装の見出しを取り出す(字句一覧: &[TokenTree], 添字: usize) -> 見出しの中身 {
    let (型引数の名前一覧, 後ろ) = 先頭の型引数を分ける(字句一覧.get(添字 + 1..).unwrap_or_default());
    let 宣言 = 最上位で止める(後ろ, |字句| matches!(字句, TokenTree::Group(群) if 群.delimiter() == Delimiter::Brace) || 語か(字句, "where"));
    let forの位置 = 最上位の位置(宣言, |位置| 語か(&宣言[位置], "for") && !高階の寿命の束縛か(宣言.get(位置 + 1..).unwrap_or_default()));
    let (トレイトの実装か, 対象) = forの位置.map_or((false, 宣言), |位置| (true, 宣言.get(位置 + 1..).unwrap_or_default()));
    見出しの中身::実装 {
        トレイトの実装か,
        対象の型の名前: 対象の型の名前(対象),
        型引数の名前一覧,
    }
}

/// 並びの添字の `type` の後ろの字句から、型の別名の見出しの中身を取り出す。宣言は最上位の `;` までである。`type` の後ろが識別子でなければ無い。
pub fn 型の別名の見出しを取り出す(字句一覧: &[TokenTree], 添字: usize) -> Option<見出しの中身> {
    let Some(TokenTree::Ident(別名)) = 字句一覧.get(添字 + 1) else {
        return None;
    };
    let (型引数の名前一覧, 後ろ) = 先頭の型引数を分ける(字句一覧.get(添字 + 2..).unwrap_or_default());
    let 宣言 = 最上位で止める(後ろ, |字句| 記号か(字句, ';'));
    Some(match 最上位の位置(宣言, |位置| 記号か(&宣言[位置], '=')) {
        None => 見出しの中身::右辺の無い型の別名,
        Some(位置) => 見出しの中身::右辺のある型の別名 {
            別名: 先頭の識別子(&別名.to_string()),
            型引数の名前一覧,
            右辺の最後の名前: パスの最後の名前(宣言.get(位置 + 1..).unwrap_or_default()),
        },
    })
}

// 先頭が `<` なら、閉じる `>` までの型引数の名前の一覧と、その後ろの字句。先頭が `<` でなければ、空の一覧と字句のすべて。
fn 先頭の型引数を分ける(字句一覧: &[TokenTree]) -> (Vec<String>, &[TokenTree]) {
    if !字句一覧.first().is_some_and(|字句| 記号か(字句, '<')) {
        return (Vec::new(), 字句一覧);
    }
    let 閉じる位置 = 最上位の位置(字句一覧, |位置| 位置 > 0 && 記号か(&字句一覧[位置], '>')).unwrap_or(字句一覧.len());
    let 中 = 字句一覧.get(1..閉じる位置).unwrap_or_default();
    let 名前一覧 = 最上位のカンマで分ける(中).into_iter().filter_map(|引数| match 引数.first() {
        Some(TokenTree::Ident(名前)) if 名前 != "const" => Some(先頭の識別子(&名前.to_string())).filter(|名前| !名前.is_empty()),
        Some(_) | None => None,
    });
    (名前一覧.collect(), 字句一覧.get(閉じる位置 + 1..).unwrap_or_default())
}
