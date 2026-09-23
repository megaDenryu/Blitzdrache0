//! 1つのファイルの原文を proc-macro2 で字句の木へ変え、項目を始めうる予約語(`impl`・`type`・`use`・`macro_rules!`)の現れと、行の頭の `impl` と `type` の見出しの中身(`header_tokens.rs`)と、生の識別子の現れと、`extern crate` の宣言を数える走査。
//! 受け取るのは原文、返すのは数えた現れの一覧か、字句の木へ変えられなかった位置か、シバンで始まることである。
//! 字句の規則(識別子の文字・コメント・文字列・寿命・生の識別子・括弧の対応)は、proc-macro2 が rustc と同じ規則で読む。自前の字句の走査が、識別子の文字の定義とコメントを挟んだ字句の直結とマクロの呼び出しの括弧の対応で、読み落としを繰り返したためである。
//! トークン木(マクロの呼び出しの引数の群と `macro_rules!` の本体の群)の中では、マクロが字句を並べ替えて展開できるため、すべての現れを数える。`if !(..)` の群もトークン木に数えるが、厳しくなる向きの誤りだけである。
//! トークン木の外では、同じ並びの直前の字句が項目を始めうるときだけ数える(`item_keyword_position.rs`)。行の頭かどうかでは除かず、行の頭かを現れごとに持つ(`line_head.rs`)。読み口の答えとの突き合わせは `item_keyword_reconciliation.rs` が行う。
//! 寿命(`'impl`)の識別子と生の識別子(`r#impl`)は予約語として数えない。マクロのメタ変数の `$` の後ろの予約語は除かない。`捨てる!($ impl 型 { .. })` のように、`$` の後ろに書いた実装を展開できるためである。
//! バイト順マークを除いた先頭が `#!` で3文字目が `[` でないファイルは、数えずにシバンで始まると答える。rustc はシバンの行を読み飛ばすが proc-macro2 は読み飛ばさない。
//! 読み飛ばす規則を真似ると、`#!/*` で始まるファイルと `#! [..] impl ..` の1行が2つの読みの食い違いになるため、シバンを禁止して違反にする(`readable_form_assertion.rs`)。

use std::str::FromStr;

use proc_macro2::{Ident, TokenStream, TokenTree};

use super::enclosing_group::並びを囲む群;
use super::header_content::見出しの中身;
use super::header_tokens::{型の別名の見出しを取り出す, 実装の見出しを取り出す};
use super::item_keyword_position::{予約語の現れた場所, 項目の予約語};
use super::line_head::行の頭か;
use super::preceding_token::直前の字句;

/// 1つのファイルを字句の木へ変えて数えた結末。
pub enum ファイルの字句の木 {
    数えた(字句の木から数えた現れ),
    変えられない { 行番号: usize, 桁: usize },
    シバンで始まる,
}

/// 字句の木から数えた、項目を始めうる予約語の現れと、生の識別子の現れと、`extern crate` の宣言の行。並びは原文の中の順である。
#[derive(Default)]
pub struct 字句の木から数えた現れ {
    pub 予約語の現れ一覧: Vec<項目を始めうる予約語の現れ>,
    pub 生の識別子一覧: Vec<生の識別子の現れ>,
    pub 外部クレートの宣言の行一覧: Vec<usize>, // `extern` の直後に `crate` が続く字句の行(1始まり)
    pub 行の頭の見出し一覧: Vec<行の頭の見出し>,
}

/// 行の頭の `impl` と `type` の後ろの字句から取り出した見出しの中身1件。行番号は1始まりである。
pub struct 行の頭の見出し {
    pub 行番号: usize,
    pub 予約語: 項目の予約語,
    pub 中身: 見出しの中身,
}

/// 項目を始めうる予約語の現れ1件。行番号は1始まりである。
pub struct 項目を始めうる予約語の現れ {
    pub 行番号: usize,
    pub 予約語: 項目の予約語,
    pub 場所: 予約語の現れた場所,
    pub 行の頭か: bool,
}

/// 生の識別子(`r#名前`)の現れ1件。名前は `r#` を外したものである。
pub struct 生の識別子の現れ {
    pub 行番号: usize,
    pub 名前: String,
}

impl ファイルの字句の木 {
    /// 原文を字句の木へ変えて数える。変えられなければ、変えられなかった位置(1始まりの行と桁)を返す。
    pub fn 原文から数える(原文: &str) -> Self {
        let 本文 = 原文.strip_prefix('\u{feff}').unwrap_or(原文);
        if 本文.starts_with("#!") && !本文.starts_with("#![") {
            return Self::シバンで始まる;
        }
        match TokenStream::from_str(本文) {
            Ok(木) => {
                let mut 数えた = 字句の木から数えた現れ::default();
                数えた.並びを読む(木, 並びを囲む群::ファイルの直下());
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
    // 1つの並びを読み、群の中へは入れ子ごと入る。
    fn 並びを読む(&mut self, 並び: TokenStream, 囲み: 並びを囲む群) {
        let 字句一覧: Vec<TokenTree> = 並び.into_iter().collect();
        for (添字, 字句) in 字句一覧.iter().enumerate() {
            match 字句 {
                TokenTree::Group(群) => self.並びを読む(群.stream(), 囲み.中の群を囲む(&字句一覧, 添字, 群)),
                TokenTree::Ident(識別子) => self.識別子を読む(&字句一覧, 添字, 識別子, 囲み),
                TokenTree::Punct(_) | TokenTree::Literal(_) => {}
            }
        }
    }

    fn 識別子を読む(&mut self, 字句一覧: &[TokenTree], 添字: usize, 識別子: &Ident, 囲み: 並びを囲む群) {
        let 行番号 = 識別子.span().start().line;
        if 識別子 == "extern" && matches!(字句一覧.get(添字 + 1), Some(TokenTree::Ident(語)) if 語 == "crate") {
            self.外部クレートの宣言の行一覧.push(行番号);
        }
        if let Some(名前) = 識別子.to_string().strip_prefix("r#") {
            self.生の識別子一覧.push(生の識別子の現れ { 行番号, 名前: 名前.to_string() });
            return;
        }
        let 直前 = 添字.checked_sub(1).map_or(囲み.最初の字句の直前, |前| 直前の字句::読む(字句一覧, 前));
        let Some(予約語) = 項目の予約語::並びの中で読む(字句一覧, 添字).filter(|_| 直前 != 直前の字句::寿命の引用符) else {
            return;
        };
        let 場所 = match (囲み.トークン木の中か, 直前) {
            (true, 直前の字句::ドル記号) => 予約語の現れた場所::マクロのメタ変数の位置,
            (true, _) => 予約語の現れた場所::トークン木の中,
            (false, _) if 予約語.項目を始めうる直前か(直前) => 予約語の現れた場所::トークン木の外で項目を始めうる位置,
            (false, _) => return,
        };
        let 行の頭か = 行の頭か(字句一覧, 添字, 予約語, 囲み.開いた行);
        if 行の頭か {
            let 中身 = match 予約語 {
                項目の予約語::実装 => Some(実装の見出しを取り出す(字句一覧, 添字)),
                項目の予約語::型の別名 => 型の別名の見出しを取り出す(字句一覧, 添字),
                項目の予約語::取り込み | 項目の予約語::マクロの定義 => None,
            };
            self.行の頭の見出し一覧.extend(中身.map(|中身| 行の頭の見出し { 行番号, 予約語, 中身 }));
        }
        self.予約語の現れ一覧.push(項目を始めうる予約語の現れ { 行番号, 予約語, 場所, 行の頭か });
    }
}
