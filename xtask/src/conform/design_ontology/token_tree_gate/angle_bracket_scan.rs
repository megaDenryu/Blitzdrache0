//! 字句の木の1つの並びを、山括弧の外で探す純粋な関数。受け取るのは proc-macro2 の字句の並び、返すのは当たった位置か、分けた並びである。
//! 括弧(丸括弧・角括弧・波括弧)の中は群として1つの字句になるため、最上位かどうかは山括弧の深さだけで決まる。`->` の `>` は山括弧を閉じない(`declaration_brackets.rs` と同じ規則)。
//! 見出しの中身の取り出し(`header_tokens.rs`)と型の表記の名前(`type_notation_name.rs`)が、見出しを本体と `where` と `;` で止め、型引数をカンマで分け、`for` と `=` を探すときに使う。

use proc_macro2::{Spacing, TokenTree};

/// 字句の並びを、山括弧の外のカンマで分ける。
pub fn 最上位のカンマで分ける(字句一覧: &[TokenTree]) -> Vec<&[TokenTree]> {
    let mut 引数一覧 = Vec::new();
    let mut 残り = 字句一覧;
    while let Some(位置) = 最上位の位置(残り, |位置| 記号か(&残り[位置], ',')) {
        引数一覧.push(&残り[..位置]);
        残り = 残り.get(位置 + 1..).unwrap_or_default();
    }
    引数一覧.push(残り);
    引数一覧
}

/// 字句の並びの、山括弧の外で `止めるか` が当たる最初の字句の手前まで。当たらなければ並びのすべて。
pub fn 最上位で止める(字句一覧: &[TokenTree], 止めるか: impl Fn(&TokenTree) -> bool) -> &[TokenTree] {
    最上位の位置(字句一覧, |位置| 止めるか(&字句一覧[位置])).map_or(字句一覧, |位置| &字句一覧[..位置])
}

/// 山括弧の外で `当たるか` が当たる最初の位置。`<` と `>` の字句そのものは、深さを進める前の深さで問う。
pub fn 最上位の位置(字句一覧: &[TokenTree], 当たるか: impl Fn(usize) -> bool) -> Option<usize> {
    let mut 深さ = 0usize;
    for 位置 in 0..字句一覧.len() {
        let 閉じるか = 記号か(&字句一覧[位置], '>') && !矢印の後ろか(字句一覧, 位置);
        if 閉じるか {
            深さ = 深さ.saturating_sub(1);
        }
        if 深さ == 0 && 当たるか(位置) {
            return Some(位置);
        }
        if 記号か(&字句一覧[位置], '<') {
            深さ += 1;
        }
    }
    None
}

// 添字の `>` が `->` の `>` か。
fn 矢印の後ろか(字句一覧: &[TokenTree], 添字: usize) -> bool {
    添字
        .checked_sub(1)
        .and_then(|前| 字句一覧.get(前))
        .is_some_and(|字句| matches!(字句, TokenTree::Punct(記号) if 記号.as_char() == '-' && 記号.spacing() == Spacing::Joint))
}

/// `for` の後ろが高階の寿命の束縛の山括弧(`<'a>`・`<>`)か。
pub fn 高階の寿命の束縛か(後ろ: &[TokenTree]) -> bool {
    後ろ.first().is_some_and(|字句| 記号か(字句, '<')) && 後ろ.get(1).is_some_and(|字句| 記号か(字句, '\'') || 記号か(字句, '>'))
}

pub fn 記号か(字句: &TokenTree, 文字: char) -> bool {
    matches!(字句, TokenTree::Punct(記号) if 記号.as_char() == 文字)
}

pub fn 語か(字句: &TokenTree, 語: &str) -> bool {
    matches!(字句, TokenTree::Ident(識別子) if 識別子 == 語)
}
