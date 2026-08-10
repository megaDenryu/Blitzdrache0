//! 角速度の単位型。次元の合成（角速度 × 時間 = 角）を持つ。

use std::ops::Mul;

use super::ラジアン;
use super::秒;

/// ラジアン毎秒単位の角速度。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ラジアン毎秒(f32);

impl ラジアン毎秒 {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 度毎秒で書いた角速度から生成する。人が回る速さを決めるときは度のほうが取り違えにくい。
    pub fn 度毎秒から(度毎秒: f32) -> Self {
        Self(度毎秒.to_radians())
    }
}

/// 次元の合成: 角速度 × 時間 = 角。
impl Mul<秒> for ラジアン毎秒 {
    type Output = ラジアン;
    fn mul(self, 経過時間: 秒) -> ラジアン {
        ラジアン::生成する(self.0 * 経過時間.値())
    }
}
