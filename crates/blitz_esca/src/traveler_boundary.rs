//! 旅行者が移動可能な世界の境界定義(Issue #137)。
//!
//! 世界のルールとして旅行者の行動を律する空間的制約と判定結果を定義する。

use blitz_math::メートル;
use crate::ontology::MDTO;

/// 境界による座標制限の計算結果DTO。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 境界制限結果 {
    pub 制限後の東: メートル,
    pub 制限後の北: メートル,
    pub 衝突した: bool,
}

impl MDTO for 境界制限結果 {}

/// 旅行者が歩行可能な空間の東西南北の限界範囲DTO。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 移動可能範囲 {
    pub 東の最小: メートル,
    pub 東の最大: メートル,
    pub 北の最小: メートル,
    pub 北の最大: メートル,
}

impl MDTO for 移動可能範囲 {}

impl 移動可能範囲 {
    /// 指定した座標を境界内に収め、計算結果DTOを返す。
    pub fn 制限する(&self, 東: メートル, 北: メートル) -> 境界制限結果 {
        let 元の東 = 東.値();
        let 元の北 = 北.値();
        let 制限東 = 元の東.clamp(self.東の最小.値(), self.東の最大.値());
        let 制限北 = 元の北.clamp(self.北の最小.値(), self.北の最大.値());
        let 衝突 = (制限東 - 元の東).abs() > 1e-6 || (制限北 - 元の北).abs() > 1e-6;

        境界制限結果 {
            制限後の東: メートル::生成する(制限東),
            制限後の北: メートル::生成する(制限北),
            衝突した: 衝突,
        }
    }
}
