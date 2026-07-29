//! ゲーム時計が実時間を世界時間へ写す倍率。

use super::時刻エラー;

/// 実時間1秒あたり世界時間が何秒進むかの倍率。0は進まないことを表し、負の倍率は認めない。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 時間倍率(f64);

impl 時間倍率 {
    pub fn 生成する(値: f64) -> Result<Self, 時刻エラー> {
        if !値.is_finite() || 値 < 0.0 {
            return Err(時刻エラー::時間倍率が負または非有限);
        }
        Ok(Self(値))
    }

    /// 実時間と世界時間が同じ速さで進む倍率。
    pub fn 等速() -> Self {
        Self(1.0)
    }

    pub fn 値(&self) -> f64 {
        self.0
    }
}
