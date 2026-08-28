//! コマンドが受け取る引数を型で表す一式。担当するのは、引数の形・説明・省略したときの扱い・
//! 選べる綴りを値として持つ型と、使い方の行が見せる引数の構文を持つ型を提供することだけである。
//! どのコマンドがどの引数を持つかは分類ごとの一覧が持ち、その定義を人へ尋ねるのは`menu`の
//! 引数フォームが持つ。

mod choice;
mod definition;
mod omission;
mod syntax;

pub(crate) use choice::選択肢;
pub(crate) use definition::引数定義;
pub(crate) use omission::省略したときの扱い;
pub(crate) use syntax::引数の構文;
