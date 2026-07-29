//! 時刻から導く方向光の強度。

use super::天空状態エラー;

/// 太陽光の強度。有限かつ0以上であり、0は方向光が働かない状態を表す。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 太陽光強度(f32);

impl 太陽光強度 {
    pub fn 生成する(値: f32) -> Result<Self, 天空状態エラー> {
        if !値.is_finite() || 値 < 0.0 {
            return Err(天空状態エラー::値域外("太陽光強度", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }

    /// 0以上1以下の係数で弱める。太陽が地平線帯を降りる間に強度を連続して落とすために使う。
    pub fn 弱める(&self, 係数: f32) -> Result<Self, 天空状態エラー> {
        if !係数.is_finite() || !(0.0..=1.0).contains(&係数) {
            return Err(天空状態エラー::値域外("太陽光強度を弱める係数", 係数));
        }
        Self::生成する(self.0 * 係数)
    }
}
