//! 部品とカタログ。1つの部品が持つ接合点の束と、全部品を集めた台帳を表す型を集める。
//!
//! この束は`joint`を知ってよいが、噛み合わせの計算(`mating`)も展開(`expansion`)も知らない。
//! カタログは全部品を読み終えるまで不変条件を検査できないため、組み立て器を通してのみ完成させる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「部品カタログと展開器」

mod bounding_box;
mod catalog;
mod catalog_builder;
mod corner_transform;
mod definition;
mod error;
mod part_id;
mod stored_part;

#[cfg(test)]
mod part_fixture;
#[cfg(test)]
mod part_tests;

pub use bounding_box::{群ローカルの箱, 部品の境界箱};
pub use catalog::{接合種別ごとの員数, 部品カタログ};
pub use catalog_builder::部品カタログ組み立て器;
pub use definition::部品定義;
pub use error::{カタログエラー, 対の不足};
pub use part_id::部品ID;
pub use stored_part::収蔵部品;
