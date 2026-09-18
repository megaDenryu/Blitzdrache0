//! 遷移に与える経過時間。有限かつ0以上を不変条件に持つ値オブジェクトであり、負の値や NaN・無限大で現在地を汚さないために置く。

use blitz_design::MDTO;
use blitz_math::秒;

/// 遷移関数へ渡す、有限かつ0以上の経過時間。`生成する` を通らずには作れない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 経過時間(秒);

impl MDTO for 経過時間 {}

impl 経過時間 {
    /// 値が有限かつ0以上であることを確かめて経過時間を作る。
    pub fn 生成する(値: 秒) -> Result<Self, 経過時間の生成の失敗> {
        let 生値 = 値.値();
        if !生値.is_finite() || 生値 < 0.0 {
            return Err(経過時間の生成の失敗::有限な0以上の数値でない { 値 });
        }
        Ok(Self(値))
    }

    pub fn 秒(&self) -> 秒 {
        self.0
    }
}

/// 経過時間の値が不変条件を満たさなかった理由。
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum 経過時間の生成の失敗 {
    #[error("経過時間{値:?}が有限な0以上の数値でない(経過時間は有限かつ0以上でなければならない)")]
    有限な0以上の数値でない { 値: 秒 },
}
