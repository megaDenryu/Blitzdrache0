//! 判定が比べる値: 検収の判定が実測と期待の両側へ置く値。件数と実数と綴りの3種を数え上げる。
//!
//! 破れの型が実測と期待を文へ焼いてから持つと、「上限{下限}を超えた」のような役割の取り違えが文面にしか現れず、
//! 読み手はどちらの数が実測なのかを復元できない。値を型のまま持ち、文へ写すのは`Display`の1箇所だけにする。
//!
//! どのRustの型を判定の材料にしてよいかの数え上げは`comparable`が持つ。

mod comparable;

use std::fmt;

pub use comparable::判定で比べられる値;

#[derive(Debug, Clone, PartialEq)]
pub enum 判定が比べる値 {
    件数(u64),
    実数(f64),
    綴り(String),
}

impl fmt::Display for 判定が比べる値 {
    fn fmt(&self, 書き手: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::件数(値) => write!(書き手, "{値}"),
            Self::実数(値) => write!(書き手, "{値}"),
            Self::綴り(値) => write!(書き手, "{値}"),
        }
    }
}
