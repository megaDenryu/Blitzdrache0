//! 字句の木の中の項目の予約語の現れが、行の頭にあるかを答える純粋な関数。受け取るのは proc-macro2 の字句の並びと現れの添字と予約語の種類と、並びを囲む群を開いた行、返すのは行の頭にあるかである。
//! 行の頭とは、字下げと属性と可視性(`impl` では可視性の代わりに `unsafe`)を読み飛ばした位置のことであり、4つの読み口が宣言を読むのはこの位置だけである(`reader_answer.rs`)。
//! 現れの前の字句を同じ並びの中で遡り、同じ行にある字句が読み飛ばせる前置き(`#[..]`・`#![..]`・`impl` では `unsafe`・それ以外では `pub` と `pub(..)`)だけなら行の頭である。
//! 前の行で終わる字句に届けば行の頭であり、並びの始めに届けば、並びを囲む群を開いた括弧が前の行にあるとき(ファイルの直下なら常に)行の頭である。
//! 前置きを予約語ごとに分けるのは、読み口が読み飛ばす前置きと同じにするためである(`line_matching.rs` の `implの予約語より後ろ`・`declaration_prefix.rs`)。

use proc_macro2::{Delimiter, TokenTree};

use super::item_keyword_position::項目の予約語;

/// 並びの添字の予約語の現れが行の頭にあるか。`群を開いた行` は並びを囲む群を開いた括弧の行(1始まり)であり、ファイルの直下の並びなら無い。
pub fn 行の頭か(字句一覧: &[TokenTree], 添字: usize, 予約語: 項目の予約語, 群を開いた行: Option<usize>) -> bool {
    let Some(現れ) = 字句一覧.get(添字) else {
        return false;
    };
    let 行 = 現れ.span().start().line;
    let mut 位置 = 添字;
    while let Some(前) = 位置.checked_sub(1) {
        let Some(字句) = 字句一覧.get(前) else {
            return false;
        };
        if 字句.span().end().line < 行 {
            return true;
        }
        if 字句.span().start().line < 行 || !読み飛ばせる前置きか(字句一覧, 前, 予約語) {
            return false;
        }
        位置 = 前;
    }
    群を開いた行.is_none_or(|開いた行| 開いた行 < 行)
}

// 添字の字句が、読み口が予約語の前で読み飛ばす前置き(属性の `#`・`!`・角括弧の群と、予約語ごとの可視性か `unsafe`)の一部か。
fn 読み飛ばせる前置きか(字句一覧: &[TokenTree], 添字: usize, 予約語: 項目の予約語) -> bool {
    let 前 = |ずれ: usize| 添字.checked_sub(ずれ).and_then(|位置| 字句一覧.get(位置));
    let 記号か = |字句: Option<&TokenTree>, 文字: char| matches!(字句, Some(TokenTree::Punct(記号)) if 記号.as_char() == 文字);
    let 語か = |字句: Option<&TokenTree>, 語: &str| matches!(字句, Some(TokenTree::Ident(識別子)) if 識別子 == 語);
    let 可視性を読み飛ばすか = matches!(予約語, 項目の予約語::型の別名 | 項目の予約語::取り込み);
    match 字句一覧.get(添字) {
        Some(TokenTree::Punct(記号)) => 記号.as_char() == '#' || (記号.as_char() == '!' && 記号か(前(1), '#')),
        Some(TokenTree::Group(群)) => match 群.delimiter() {
            Delimiter::Bracket => 記号か(前(1), '#') || (記号か(前(1), '!') && 記号か(前(2), '#')),
            Delimiter::Parenthesis => 可視性を読み飛ばすか && 語か(前(1), "pub"),
            Delimiter::Brace | Delimiter::None => false,
        },
        Some(TokenTree::Ident(識別子)) => (予約語 == 項目の予約語::実装 && 識別子 == "unsafe") || (可視性を読み飛ばすか && 識別子 == "pub"),
        Some(TokenTree::Literal(_)) | None => false,
    }
}
