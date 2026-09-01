//! 加速度の単位型。次元の合成(加速度 × 時間 = 速度)を持つ。重力で鉛直の速度を進める計算が最初の利用者である。

use std::ops::Mul;

use super::メートル毎秒;
use super::秒;

/// メートル毎秒毎秒単位の加速度。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct メートル毎秒毎秒(f32);

impl メートル毎秒毎秒 {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

/// 次元の合成: 加速度 × 時間 = 速度。
impl Mul<秒> for メートル毎秒毎秒 {
    type Output = メートル毎秒;
    fn mul(self, 経過時間: 秒) -> メートル毎秒 {
        メートル毎秒::生成する(self.0 * 経過時間.値())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 加速度に時間を掛けると速度になる() {
        assert_eq!(メートル毎秒毎秒::生成する(9.8) * 秒::生成する(2.0), メートル毎秒::生成する(19.6));
    }
}
