//! 字句の木の型の表記から、パスの最後の名前を読む純粋な関数。受け取るのは型の表記の字句、返すのは1つの識別子で書ける名前か、名前で読めないときの無しである。
//! 実装の対象の型は、1つの型を囲む丸括弧と、参照(`&`・`&'a`・`&mut`)と、`Pin<&mut ..>` を、読み口(`impl_syntax/target_type.rs`)と同じ順で外してから名前を読む。
//! 名前は `<` か丸括弧の群の手前までのパスの最後の `::` より後ろであり、読み口の `line_matching.rs` の `パスの最後の名前` と同じ位置を取る。

use proc_macro2::{Delimiter, TokenTree};

use super::super::line_matching::名前で読める表記;
use super::angle_bracket_scan::{最上位の位置, 記号か, 語か};

/// 対象の型の字句から、囲む丸括弧と参照と `Pin<&mut ..>` を外した型のパスの最後の名前。
pub fn 対象の型の名前(字句一覧: &[TokenTree]) -> Option<String> {
    let 外した = 丸括弧を外す(字句一覧);
    let 参照先: &[TokenTree] = match 参照を外す(&外した).or_else(|| pinの中身(&外した).and_then(|中身| 参照を外す(中身).filter(|(_, 可変か)| *可変か))) {
        Some((参照先, _)) => 参照先,
        None => &外した,
    };
    パスの最後の名前(&丸括弧を外す(参照先))
}

// 字句の並びが、中が1つの型である丸括弧の群だけなら、中を外す。外せなくなるまで繰り返す。単位型とタプルは外さない。
fn 丸括弧を外す(字句一覧: &[TokenTree]) -> Vec<TokenTree> {
    let mut 並び = 字句一覧.to_vec();
    while let [TokenTree::Group(群)] = 並び.as_slice() {
        let 中: Vec<TokenTree> = 群.stream().into_iter().collect();
        if 群.delimiter() != Delimiter::Parenthesis || 中.is_empty() || 最上位の位置(&中, |位置| 記号か(&中[位置], ',')).is_some() {
            break;
        }
        並び = 中;
    }
    並び
}

// 参照の型(`&X`・`&'a X`・`&mut X`)の参照先の字句と、可変参照か。参照の型でなければ無い。
fn 参照を外す(字句一覧: &[TokenTree]) -> Option<(&[TokenTree], bool)> {
    let (先頭, 後ろ) = 字句一覧.split_first()?;
    if !記号か(先頭, '&') {
        return None;
    }
    let 後ろ = if 後ろ.first().is_some_and(|字句| 記号か(字句, '\'')) { 後ろ.get(2..)? } else { 後ろ };
    Some(match 後ろ.split_first() {
        Some((字句, 参照先)) if 語か(字句, "mut") => (参照先, true),
        Some(_) | None => (後ろ, false),
    })
}

// `Pin<X>`(`std::pin::Pin<X>` を含む)の中身 `X` の字句。`Pin` でなければ無い。
fn pinの中身(字句一覧: &[TokenTree]) -> Option<&[TokenTree]> {
    let 開く位置 = 字句一覧.iter().position(|字句| 記号か(字句, '<'))?;
    let 閉じている = 字句一覧.last().is_some_and(|字句| 記号か(字句, '>'));
    (閉じている && パスの最後の名前(字句一覧.get(..開く位置)?).as_deref() == Some("Pin"))
        .then(|| 字句一覧.get(開く位置 + 1..字句一覧.len() - 1))
        .flatten()
}

/// 型のパスの最後の名前。`<` か丸括弧の群の手前までを取り、最後の `::` より後ろが1つの識別子(頭に `$` があってもよい)なら、その名前。
pub fn パスの最後の名前(字句一覧: &[TokenTree]) -> Option<String> {
    let パス = 字句一覧
        .iter()
        .position(|字句| 記号か(字句, '<') || matches!(字句, TokenTree::Group(群) if 群.delimiter() == Delimiter::Parenthesis))
        .map_or(字句一覧, |位置| &字句一覧[..位置]);
    let 最後の区切り = (1..パス.len()).rev().find(|位置| 記号か(&パス[位置 - 1], ':') && 記号か(&パス[*位置], ':'));
    let 最後の要素 = 最後の区切り.map_or(パス, |位置| &パス[位置 + 1..]);
    let 表記 = match 最後の要素 {
        [TokenTree::Ident(名前)] => 名前.to_string(),
        [TokenTree::Punct(記号), TokenTree::Ident(名前)] if 記号.as_char() == '$' => format!("${名前}"),
        [..] => return None,
    };
    名前で読める表記(&表記)
}
