//! 時刻から導く露出の補正量。

use super::天空状態エラー;

/// 2を底とする段数で表す露出の補正量。最終倍率は露出倍率×2の露出補正段乗であり、合成はコンポジションルートが行う。
/// 正の段は明るく、負の段は暗くする。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 露出補正段(f32);

impl 露出補正段 {
    pub fn 生成する(値: f32) -> Result<Self, 天空状態エラー> {
        if !値.is_finite() {
            return Err(天空状態エラー::値域外("露出補正段", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }

    /// 自身を係数0、相手を係数1として線形に混ぜる。
    pub fn 混ぜる(&self, 相手: &Self, 係数: f32) -> Result<Self, 天空状態エラー> {
        Self::生成する(self.0 + (相手.0 - self.0) * 係数)
    }
}
