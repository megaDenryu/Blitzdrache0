//! 字句の木の1つの並びを囲む群。並びがトークン木の中にあるかと、並びの最初の字句の直前の区分と、群を開いた括弧の行を持ち、中の群を囲む群を自分で組む。
//! 字句の木の走査(`token_tree_scan.rs`)は、予約語の現れの場所をトークン木の中かで分け、行の頭の判定(`line_head.rs`)は並びの始めに届いたときに群を開いた行を見る。
//! トークン木とは、マクロの呼び出しの引数(`名前 ! 群`)と `macro_rules! 名前` の本体の群のことであり、トークン木の中の群はすべてトークン木の中にある。

use proc_macro2::{Group, TokenTree};

use super::preceding_token::直前の字句;

/// 字句の木の1つの並びを囲む群。ファイルの直下の並びは、トークン木の外にあり、項目の区切りの後ろから始まり、開いた行を持たない。
#[derive(Clone, Copy)]
pub struct 並びを囲む群 {
    pub トークン木の中か: bool,
    pub 最初の字句の直前: 直前の字句,
    pub 開いた行: Option<usize>, // 群を開いた括弧の行(1始まり)
}

impl 並びを囲む群 {
    /// ファイルの直下の並びを囲むもの。
    pub const fn ファイルの直下() -> Self {
        Self {
            トークン木の中か: false,
            最初の字句の直前: 直前の字句::項目の区切り,
            開いた行: None,
        }
    }

    /// この並びの添字にある群の、中の並びを囲むもの。
    pub fn 中の群を囲む(self, 字句一覧: &[TokenTree], 添字: usize, 群: &Group) -> Self {
        Self {
            トークン木の中か: self.トークン木の中か || マクロの引数の群か(字句一覧, 添字),
            最初の字句の直前: 直前の字句::群を開いた直後(群.delimiter()),
            開いた行: Some(群.span_open().start().line),
        }
    }
}

// 添字の群が、マクロの呼び出しの引数(`名前 ! 群`)か `macro_rules! 名前 群` の本体か。
fn マクロの引数の群か(字句一覧: &[TokenTree], 添字: usize) -> bool {
    let 前 = |ずれ: usize| 添字.checked_sub(ずれ).and_then(|位置| 字句一覧.get(位置));
    let 識別子か = |字句: Option<&TokenTree>| matches!(字句, Some(TokenTree::Ident(_)));
    let 感嘆符か = |字句: Option<&TokenTree>| matches!(字句, Some(TokenTree::Punct(記号)) if 記号.as_char() == '!');
    let 呼び出しか = 感嘆符か(前(1)) && 識別子か(前(2));
    let 定義か = 識別子か(前(1)) && 感嘆符か(前(2)) && matches!(前(3), Some(TokenTree::Ident(語)) if 語 == "macro_rules");
    呼び出しか || 定義か
}
