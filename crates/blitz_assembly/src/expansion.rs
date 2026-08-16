//! 手順から部品ごとの配置表を作る展開。カタログと組み立て手順を受け取り、描画対象の単位になる配置表を返す。
//!
//! この束が知ってよいのは`joint`と`part`と`mating`と`blitz_engine`の`個体配置`だけである。
//! glTFもファイルも時刻も乱数の外部の源も知らない純粋計算であり、同じ入力からは常に同じ配置表が出る。
//!
//! **段2の手順はまだ乱数の種を持たない。** 段3で規則を導入し、壁の面ごとに窓壁か扉壁かを種から選ぶようになったとき、
//! 規則がこの手順を生む形になる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「部品カタログと展開器」

mod error;
mod expander;
mod instruction;
mod placed_parts;
mod placement_table;

#[cfg(test)]
mod expansion_fixture;
#[cfg(test)]
mod expansion_rejection_tests;
#[cfg(test)]
mod expansion_tests;

pub use error::展開エラー;
pub use expander::展開器;
pub use instruction::{据えた部品の番号, 接合の指示, 組み立て手順};
pub use placement_table::{据えた配置, 部品ごとの配置, 部品ごとの配置表};
