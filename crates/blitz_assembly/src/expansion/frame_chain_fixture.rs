//! 骨格を連ねた組み立ての試験の材料のうち、組み上がった配置を読み出す口と近さの判定、およびはめ口一覧の組み方。
//! 連なりの規則そのものは`frame_chain_rule_fixture`が持つ。
//!
//! 規則と分けるのは、変わる理由が違うためである。読み出しの口は配置表の形が変わると変わり、
//! 規則は候補の並べ方を変えると変わる。

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use blitz_engine::個体配置;

use super::frame_bay_joint_fixture::はめ口の宣言一覧;
use super::frame_fixture::{名前, 識別子};
use super::frame_rule_fixture::壁をはめる候補;
use super::opening_choice::はめ口の指定;
use super::placement_table::部品ごとの配置表;

/// 骨格の部品の綴り。試験が骨格と壁と屋根を数え分けるために使う。
pub(super) const 骨格の綴り: &str = "一間四方の骨格";
pub(super) const 屋根の綴り: &str = "切妻屋根";
pub(super) const 床板の綴り: &str = "床板";

/// 配置表から、名指した部品の配置を据えた順に取り出す。
pub(super) fn 綴りごとの配置(配置表: &部品ごとの配置表, 綴り: &str) -> Vec<個体配置> {
    配置表
        .据えた順()
        .iter()
        .filter(|据えた| 据えた.識別子() == &識別子(綴り))
        .map(|据えた| 据えた.配置())
        .collect()
}

pub(super) fn 平行移動が近いか(得た: [f32; 3], 期待: [f32; 3]) -> bool {
    得た.iter().zip(期待).all(|(得, 期)| (得 - 期).abs() < 1.0e-5)
}

pub(super) fn 恒等の姿勢か(回転: [f32; 4]) -> bool {
    回転[0].abs() < 1.0e-5 && 回転[1].abs() < 1.0e-5 && 回転[2].abs() < 1.0e-5 && (回転[3].abs() - 1.0).abs() < 1.0e-5
}

/// 名指した面へ平壁を必ず1枚入れるはめ口の指定を並べる。候補を1つにして、どの種でも同じ壁が同じ面へ入るようにする。
pub(super) fn 平壁を必ず入れるはめ口一覧(面の綴り一覧: &[&str]) -> Vec<はめ口の指定> {
    面の綴り一覧
        .iter()
        .map(|綴り| はめ口の指定::生成する(名前(綴り), vec![壁をはめる候補("平壁")]).unwrap())
        .collect()
}

pub(super) fn 四面すべてのはめ口の綴り() -> Vec<&'static str> {
    はめ口の宣言一覧.into_iter().map(|(綴り, _)| 綴り).collect()
}
