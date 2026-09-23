//! 字句の木の同じ並びの中で、項目の予約語の直前にある字句の区分。受け取るのは proc-macro2 の字句の並びと添字、返すのはその字句の区分である。
//! 区別するのは、項目を始めうるかを予約語ごとに決める判定(`item_keyword_position.rs`)に要る字句と、予約語として数えない寿命の引用符と、違反の説明に要るマクロのメタ変数の `$` だけである。
//! 群(括弧で囲んだ字句の並び)の最初の字句の直前は、群を開いた括弧である。`{` は項目の区切りであり、`(` と `[` は項目の区切りでない。

use proc_macro2::{Delimiter, Spacing, TokenTree};

/// 項目の予約語の直前にある字句の区分。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 直前の字句 {
    項目の区切り,
    可視性を閉じる丸括弧,
    予約語のunsafe,
    予約語のdefault,
    予約語のpub,
    寿命の引用符,
    ドル記号,
    その他,
}

impl 直前の字句 {
    /// 並びの添字の字句の区分。閉じた波括弧と角括弧(`}`・`]`)と `;` は項目の区切りであり、`pub` の直後の丸括弧の群(`pub(crate)`)は可視性を閉じる丸括弧である。
    pub fn 読む(字句一覧: &[TokenTree], 添字: usize) -> Self {
        match 字句一覧.get(添字) {
            Some(TokenTree::Group(群)) => match 群.delimiter() {
                Delimiter::Brace | Delimiter::Bracket => Self::項目の区切り,
                Delimiter::Parenthesis if 添字.checked_sub(1).and_then(|前| 字句一覧.get(前)).is_some_and(|前の字句| matches!(前の字句, TokenTree::Ident(語) if 語 == "pub")) => Self::可視性を閉じる丸括弧,
                Delimiter::Parenthesis | Delimiter::None => Self::その他,
            },
            Some(TokenTree::Punct(記号)) => match 記号.as_char() {
                ';' => Self::項目の区切り,
                '\'' if 記号.spacing() == Spacing::Joint => Self::寿命の引用符,
                '$' => Self::ドル記号,
                _ => Self::その他,
            },
            Some(TokenTree::Ident(語)) if 語 == "unsafe" => Self::予約語のunsafe,
            Some(TokenTree::Ident(語)) if 語 == "default" => Self::予約語のdefault,
            Some(TokenTree::Ident(語)) if 語 == "pub" => Self::予約語のpub,
            Some(TokenTree::Ident(_) | TokenTree::Literal(_)) | None => Self::その他,
        }
    }

    /// 群の最初の字句の直前の区分。波括弧の群の中は項目を始めうる位置であり、丸括弧と角括弧の群の中は項目を始めうる位置でない。
    pub const fn 群を開いた直後(区切り: Delimiter) -> Self {
        match 区切り {
            Delimiter::Brace => Self::項目の区切り,
            Delimiter::Parenthesis | Delimiter::Bracket | Delimiter::None => Self::その他,
        }
    }
}
