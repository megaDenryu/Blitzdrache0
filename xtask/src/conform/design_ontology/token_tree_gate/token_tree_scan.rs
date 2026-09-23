//! 1つのファイルの原文を proc-macro2 で字句の木へ変え、項目を始めうる予約語(`impl`・`type`・`use`・`macro_rules!`)の現れと、生の識別子の現れを数える走査。
//! 受け取るのは原文、返すのは数えた現れの一覧か、字句の木へ変えられなかった位置である。
//! 字句の規則(識別子の文字・コメント・文字列・寿命・生の識別子・括弧の対応)は、proc-macro2 が rustc と同じ規則で読む。自前の字句の走査が、識別子の文字の定義とコメントを挟んだ字句の直結とマクロの呼び出しの括弧の対応で、読み落としを繰り返したためである。
//! トークン木(マクロの呼び出しの引数の群と `macro_rules!` の本体の群)の中では、マクロが字句を並べ替えて展開できるため、すべての現れを数える。`if !(..)` の群もトークン木に数えるが、厳しくなる向きの誤りだけである。
//! トークン木の外では、同じ並びの直前の字句が項目を始めうるときだけ数える(`item_keyword_position.rs`)。行の頭かどうかでは除かない。読み口の答えとの突き合わせは `readable_form_assertion.rs` が行う。
//! 寿命(`'impl`)の識別子と生の識別子(`r#impl`)は予約語として数えない。マクロのメタ変数の `$` の後ろの予約語は除かない。`捨てる!($ impl 型 { .. })` のように、`$` の後ろに書いた実装を展開できるためである。

use std::str::FromStr;

use proc_macro2::{Ident, TokenStream, TokenTree};

use super::item_keyword_position::{予約語の現れた場所, 項目の予約語};
use super::preceding_token::直前の字句;

/// 1つのファイルを字句の木へ変えて数えた結末。
pub enum ファイルの字句の木 {
    数えた(字句の木から数えた現れ),
    変えられない { 行番号: usize, 桁: usize },
}

/// 字句の木から数えた、項目を始めうる予約語の現れと、生の識別子の現れと、`extern crate` の宣言の行。並びは原文の中の順である。
#[derive(Default)]
pub struct 字句の木から数えた現れ {
    pub 予約語の現れ一覧: Vec<項目を始めうる予約語の現れ>,
    pub 生の識別子一覧: Vec<生の識別子の現れ>,
    pub 外部クレートの宣言の行一覧: Vec<usize>, // `extern` の直後に `crate` が続く字句の行(1始まり)
}

/// 項目を始めうる予約語の現れ1件。行番号は1始まりである。
pub struct 項目を始めうる予約語の現れ {
    pub 行番号: usize,
    pub 予約語: 項目の予約語,
    pub 場所: 予約語の現れた場所,
}

/// 生の識別子(`r#名前`)の現れ1件。名前は `r#` を外したものである。
pub struct 生の識別子の現れ {
    pub 行番号: usize,
    pub 名前: String,
}

impl ファイルの字句の木 {
    /// 原文を字句の木へ変えて数える。変えられなければ、変えられなかった位置(1始まりの行と桁)を返す。
    pub fn 原文から数える(原文: &str) -> Self {
        match TokenStream::from_str(原文) {
            Ok(木) => {
                let mut 数えた = 字句の木から数えた現れ::default();
                数えた.並びを読む(木, false, 直前の字句::項目の区切り);
                Self::数えた(数えた)
            }
            Err(誤り) => {
                let 位置 = 誤り.span().start();
                Self::変えられない {
                    行番号: 位置.line, 桁: 位置.column + 1
                }
            }
        }
    }
}

impl 字句の木から数えた現れ {
    // 1つの並びを読み、群の中へは入れ子ごと入る。群の始まりは、並びの最初の字句の直前の区分である。
    fn 並びを読む(&mut self, 並び: TokenStream, トークン木の中か: bool, 群の始まり: 直前の字句) {
        let 字句一覧: Vec<TokenTree> = 並び.into_iter().collect();
        for (添字, 字句) in 字句一覧.iter().enumerate() {
            match 字句 {
                TokenTree::Group(群) => {
                    let 中か = トークン木の中か || マクロの引数の群か(&字句一覧, 添字);
                    self.並びを読む(群.stream(), 中か, 直前の字句::群を開いた直後(群.delimiter()));
                }
                TokenTree::Ident(識別子) => {
                    if 識別子 == "extern" && matches!(字句一覧.get(添字 + 1), Some(TokenTree::Ident(語)) if 語 == "crate") {
                        self.外部クレートの宣言の行一覧.push(識別子.span().start().line);
                    }
                    let 直前 = 添字.checked_sub(1).map_or(群の始まり, |前| 直前の字句::読む(&字句一覧, 前));
                    self.識別子を読む(識別子, 予約語の種類(識別子, &字句一覧, 添字), 直前, トークン木の中か);
                }
                TokenTree::Punct(_) | TokenTree::Literal(_) => {}
            }
        }
    }

    fn 識別子を読む(&mut self, 識別子: &Ident, 予約語: Option<項目の予約語>, 直前: 直前の字句, トークン木の中か: bool) {
        let 行番号 = 識別子.span().start().line;
        if let Some(名前) = 識別子.to_string().strip_prefix("r#") {
            self.生の識別子一覧.push(生の識別子の現れ { 行番号, 名前: 名前.to_string() });
            return;
        }
        let Some(予約語) = 予約語.filter(|_| 直前 != 直前の字句::寿命の引用符) else {
            return;
        };
        let 場所 = match (トークン木の中か, 直前) {
            (true, 直前の字句::ドル記号) => 予約語の現れた場所::マクロのメタ変数の位置,
            (true, _) => 予約語の現れた場所::トークン木の中,
            (false, _) if 予約語.項目を始めうる直前か(直前) => 予約語の現れた場所::トークン木の外で項目を始めうる位置,
            (false, _) => return,
        };
        self.予約語の現れ一覧.push(項目を始めうる予約語の現れ { 行番号, 予約語, 場所 });
    }
}

// 識別子が項目の予約語なら、その種類。`macro_rules` は後ろに `!` と名前が続くときだけマクロの定義である。
fn 予約語の種類(識別子: &Ident, 字句一覧: &[TokenTree], 添字: usize) -> Option<項目の予約語> {
    if 識別子 == "macro_rules" {
        let 後ろ = |ずれ: usize| 字句一覧.get(添字 + ずれ);
        return (感嘆符か(後ろ(1)) && matches!(後ろ(2), Some(TokenTree::Ident(_)))).then_some(項目の予約語::マクロの定義);
    }
    項目の予約語::表記から読む(&識別子.to_string())
}

// 添字の群が、マクロの呼び出しの引数(`名前 ! 群`)か `macro_rules! 名前 群` の本体か。
fn マクロの引数の群か(字句一覧: &[TokenTree], 添字: usize) -> bool {
    let 前 = |ずれ: usize| 添字.checked_sub(ずれ).and_then(|位置| 字句一覧.get(位置));
    let 識別子か = |字句: Option<&TokenTree>| matches!(字句, Some(TokenTree::Ident(_)));
    let 呼び出しか = 感嘆符か(前(1)) && 識別子か(前(2));
    let 定義か = 識別子か(前(1)) && 感嘆符か(前(2)) && matches!(前(3), Some(TokenTree::Ident(語)) if 語 == "macro_rules");
    呼び出しか || 定義か
}

fn 感嘆符か(字句: Option<&TokenTree>) -> bool {
    matches!(字句, Some(TokenTree::Punct(記号)) if 記号.as_char() == '!')
}
