//! 1つの噛み合わせの計算。2つの接合点が向かい合う配置を求め、既存の`個体配置`として返す。
//!
//! この束が知ってよいのは`joint`と`blitz_engine`の`個体配置`だけである。カタログも規則も知らない。
//! 部品が何であるかを知らずに、2つの接合点と親の配置だけから答えを出す。
//! 参照: `_doc/設計/部品カタログと接合点.md`「接合の定義」

mod error;
mod facing_frame;
mod fit_check;
mod joint_fit;
mod scaled_face;

#[cfg(test)]
mod mating_fixture;
#[cfg(test)]
mod mating_rejection_tests;
#[cfg(test)]
mod mating_tests;
#[cfg(test)]
mod normal_offset_tests;
#[cfg(test)]
mod rotated_root_tests;

pub use error::接合エラー;
pub use joint_fit::接合;
