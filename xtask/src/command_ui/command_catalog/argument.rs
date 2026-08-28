//! コマンドが受け取る引数を型で表す一式。担当するのは、引数の形・説明・省略したときの扱い・
//! 選べる綴りの4つを値として持つ型を提供することだけである。どのコマンドがどの引数を持つかは
//! 分類ごとの一覧が持ち、その定義を人へ尋ねるのは`menu`の引数フォームが持つ。

mod choice;
mod definition;
mod omission;

pub(crate) use choice::選択肢;
pub(crate) use definition::引数定義;
pub(crate) use omission::省略したときの扱い;
