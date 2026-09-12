//! Issue #103の段階A3と段階A4が、剛体どうしの接触(箱10段の塔と坂の上の2段の箱)について読む計器。
//! 段階A5の3つ(`increment_uncertainty`・`tower_discard_count`・`alternative_discard_tower`)も同じ場面を読むため、この木に置く。
//! 坂(剛体と静的世界の接触)の側は `crates/blitz_sim/src/contact/scene/numeric_contract_record/tolerance_dependence/eigen_direction_switch/` が持つ。
//! 剛体どうしを別に測るのは、この連立方程式が2つの動く剛体の自由度12を持ち、階数不足の向きの本数も、両側の錨が動く
//! ことによる接線変位の雑音の由来も、坂と違うためである。どれも合否は判定せず、本番の判定と閾値と既定を変えない。
//! 参照: `_doc/計測/剛体の接触の静止摩擦の許容差依存の診断_2026-09-09.md`

#![cfg(test)]

mod alternative_discard_tower;
mod eigen_relative_spread;
mod increment_uncertainty;
mod row_tolerance_by_origin;
mod tangential_noise_floor;
mod tower_discard_count;
mod tower_rest_step;
