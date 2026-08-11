//! 種の引数の読み取りが返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 「語の数が違う」「知らない語である」「数として読めない」を別の枝で持つのは、直す側が打ち直す場所が違うためである。

use super::種の選択肢の綴り;

#[derive(Debug)]
pub(super) enum 種の引数の破れ {
    引数が選択肢と値の2語でない,
    知らない引数を渡された { 綴り: String },
    種を32ビットの非負整数として読めない { 綴り: String, 誤り: std::num::ParseIntError },
}

impl std::error::Error for 種の引数の破れ {}

impl std::fmt::Display for 種の引数の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::引数が選択肢と値の2語でない => {
                write!(書き手, "引数は{種の選択肢の綴り}と種の値の2語である")
            }
            Self::知らない引数を渡された { 綴り } => write!(書き手, "知らない引数である: {綴り}"),
            Self::種を32ビットの非負整数として読めない { 綴り, 誤り } => {
                write!(書き手, "種を32ビットの非負整数として読めない({綴り}): {誤り}")
            }
        }
    }
}
