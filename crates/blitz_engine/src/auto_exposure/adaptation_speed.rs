//! 露出の補正段が1秒で動ける段数。単位はEV段毎秒である。

use super::自動露出エラー;

/// 不変条件: 有限かつ0より大きい。0では目標へ永久に近づかず、負では目標から遠ざかるため、追従の定義が壊れる。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 補正段毎秒(f32);

impl 補正段毎秒 {
    pub fn 生成する(値: f32) -> Result<Self, 自動露出エラー> {
        if !値.is_finite() || 値 <= 0.0 {
            return Err(自動露出エラー::値域外("補正段毎秒", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }
}
