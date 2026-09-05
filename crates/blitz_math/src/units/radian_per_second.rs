//! 角速度の単位型。次元の合成（角速度 × 時間 = 角）を持つ。

use std::ops::Mul;

use super::メートル;
use super::メートル毎秒;
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

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

/// 次元の合成: 角速度 × 半径 = 速さ。ラジアンは無次元であるため、回る速さに回転の中心からの長さを掛けると
/// その長さの位置にある点が持つ速さになる。
impl Mul<メートル> for ラジアン毎秒 {
    type Output = メートル毎秒;
    fn mul(self, 半径: メートル) -> メートル毎秒 {
        メートル毎秒::生成する(self.0 * 半径.値())
    }
}

/// 次元の合成: 角速度 × 時間 = 角。
impl Mul<秒> for ラジアン毎秒 {
    type Output = ラジアン;
    fn mul(self, 経過時間: 秒) -> ラジアン {
        ラジアン::生成する(self.0 * 経過時間.値())
    }
}
