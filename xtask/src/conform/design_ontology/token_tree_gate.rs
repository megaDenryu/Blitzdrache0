//! 字句の木の門番。原文を proc-macro2 で字句の木へ変えて項目を始めうる予約語を数え、4つの読み口の答えと行と種類ごとの数で突き合わせ、読み口が読まなかった宣言と読み切れなかった宣言を違反にする。
//! 受け取るのは走査範囲のファイルごとの原文(字句の木の一覧を作るとき)とコードだけの行の一覧、返すのは `クレート構文検査` の違反一覧への追加である。
//! 生の識別子(`r#名前`)が予約語の名前だけを名乗ることと、`extern crate` を宣言しないことも、同じ字句の木の上で確かめる(`raw_identifier_assertion.rs`・`extern_crate_assertion.rs`)。
//! この木の外が使うのは、字句の木の一覧(`token_tree_index.rs`)と、説明関数 `読み切れない宣言が無いこと`(`readable_form_assertion.rs`)と、工程 `予約語でない名前の生の識別子の違反一覧` と `外部クレートの宣言の違反一覧` の4つだけである。
//! 字句の数え上げ・直前の字句の区分・予約語の種類・読み口の答え・突き合わせは、この2つを組むための部品であり、木の外へ出さない。

mod extern_crate_assertion;
#[cfg(test)]
mod gatekeeper_counterexample_tests;
mod item_keyword_position;
#[cfg(test)]
mod item_keyword_position_tests;
mod item_keyword_reconciliation;
mod preceding_token;
mod raw_identifier_assertion;
mod readable_form_assertion;
mod reader_answer;
mod token_tree_index;
mod token_tree_scan;

pub use extern_crate_assertion::外部クレートの宣言の違反一覧;
pub use raw_identifier_assertion::予約語でない名前の生の識別子の違反一覧;
pub use token_tree_index::字句の木の一覧;
