//! 字句の木の門番。原文を proc-macro2 で字句の木へ変えて項目を始めうる予約語を数え、4つの読み口の答えと行の頭で双方向に完全に一致するかを突き合わせ、行の途中の宣言と、読み口が読まなかった宣言と、読み口だけが読んだ宣言と、読み切れなかった宣言を違反にする。
//! 受け取るのは走査範囲のファイルごとの原文(字句の木の一覧を作るとき)とコードだけの行の一覧、返すのは `クレート構文検査` の違反一覧への追加である。
//! 生の識別子(`r#名前`)が予約語の名前だけを名乗ることと、`extern crate` を宣言しないことと、`impl` と `type` の型引数に属性を書かないことと、実装の見出しの全体と型の別名の宣言の全体のどの深さでもマクロを呼ばないことと、型の別名と関連型の右辺に可変参照を書かないことも、
//! 同じ字句の木の上で確かめる(`raw_identifier_assertion.rs`・`extern_crate_assertion.rs`・`generic_parameter_attribute_assertion.rs`・`type_notation_macro_assertion.rs`・`alias_mutable_reference_assertion.rs`)。
//! この木の外が使うのは、字句の木の一覧(`token_tree_index.rs`)と、説明関数 `読み切れない宣言が無いこと`(`readable_form_assertion.rs`)と、工程 `予約語でない名前の生の識別子の違反一覧` と `外部クレートの宣言の違反一覧` と `型引数の属性の違反一覧` と `型の表記の中のマクロの呼び出しの違反一覧` と `型の別名の右辺の可変参照の違反一覧` の7つだけである。
//! 行の頭の `impl` と `type` の見出しの中身(トレイトの実装か・対象の型の名前と表記の全体・別名・型引数の名前・右辺の最後の名前と右辺の全体)も、字句の木から読み口と規則を共有せずに取り出し、読み口の読みと突き合わせる。字句の木は答え合わせの基準であり、構文の木は作らない。
//! 字句の数え上げ・並びを囲む群・直前の字句の区分・行の頭の判定・予約語の種類・見出しの中身・読み口の答え・突き合わせは、この2つを組むための部品であり、木の外へ出さない。

mod alias_mutable_reference_assertion;
mod angle_bracket_scan;
mod enclosing_group;
mod extern_crate_assertion;
#[cfg(test)]
mod gatekeeper_counterexample_tests;
mod generic_parameter_attribute_assertion;
mod header_content;
mod header_content_reconciliation;
#[cfg(test)]
mod header_content_tests;
mod header_tokens;
#[cfg(test)]
mod independent_reading_counterexample_tests;
mod item_keyword_position;
#[cfg(test)]
mod item_keyword_position_tests;
mod item_keyword_reconciliation;
mod line_head;
#[cfg(test)]
mod misreading_counterexample_tests;
mod normalized_token_sequence;
#[cfg(test)]
mod normalized_token_sequence_tests;
mod preceding_token;
mod raw_identifier_assertion;
mod readable_form_assertion;
mod reader_answer;
mod token_tree_index;
mod token_tree_scan;
mod type_notation_macro_assertion;
mod type_notation_name;

pub use alias_mutable_reference_assertion::型の別名の右辺の可変参照の違反一覧;
pub use extern_crate_assertion::外部クレートの宣言の違反一覧;
pub use generic_parameter_attribute_assertion::型引数の属性の違反一覧;
pub use raw_identifier_assertion::予約語でない名前の生の識別子の違反一覧;
pub use token_tree_index::字句の木の一覧;
pub use type_notation_macro_assertion::型の表記の中のマクロの呼び出しの違反一覧;
