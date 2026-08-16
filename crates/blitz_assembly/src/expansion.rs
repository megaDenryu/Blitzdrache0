//! 手順から部品ごとの配置表を作る展開。カタログと組み立て手順を受け取り、描画対象の単位になる配置表を返す。
//!
//! この束が知ってよいのは`joint`と`part`と`mating`と`blitz_engine`の`個体配置`だけである。
//! glTFもファイルも時刻も乱数の外部の源も知らない純粋計算であり、同じ入力からは常に同じ配置表が出る。
//!
//! **段2の手順はまだ乱数の種を持たない。** 段3で規則を導入し、壁の面ごとに窓壁か扉壁かを種から選ぶようになったとき、
//! 規則がこの手順を生む形になる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「部品カタログと展開器」

mod aggregated_placements;
mod error;
mod error_rule;
mod expander;
mod generation_seed;
mod house_rule;
mod instruction;
mod instruction_list_builder;
mod per_part_placements;
mod placed_parts;
mod placement_table;
mod rule;
mod step_builder;
mod tree_parts;
mod tree_rule;
mod trunk_segment;
mod wall_choice;

#[cfg(test)]
mod aggregation_tests;
#[cfg(test)]
mod expansion_fixture;
#[cfg(test)]
mod expansion_rejection_tests;
#[cfg(test)]
mod expansion_tests;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod stacking_tests;
#[cfg(test)]
mod tree_rule_fixture;
#[cfg(test)]
mod tree_rule_tests;
#[cfg(test)]
mod wall_fixture;

pub use aggregated_placements::部品ごとの配置の集約;
pub use error::展開エラー;
pub use error_rule::規則エラー;
pub use expander::展開器;
pub use generation_seed::生成の種;
pub use house_rule::{家の規則, 積む階};
pub use instruction::{据えた部品の番号, 接合の指示, 組み立て手順};
pub use per_part_placements::部品ごとの配置;
pub use placement_table::{据えた配置, 部品ごとの配置表};
pub use rule::組み立て規則;
pub use tree_parts::{幹の指定, 枝の指定, 葉房の指定};
pub use tree_rule::木の規則;
pub use trunk_segment::{幹の節数, 幹を下から数えた節の番号};
pub use wall_choice::{壁面の指定, 壁面の飾り};
