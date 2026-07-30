//! ミー散乱がどれだけ前方へ偏るかを表す1つの数。

use crate::atmosphere::大気数学エラー;

/// Cornette-Shanks位相関数の非対称係数。0が前後対称、正が前方散乱、負が後方散乱に偏る。
/// 不変条件: 有限かつ-1より大きく1より小さい。絶対値1では位相関数の分母が0になり値が発散する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 位相非対称係数(f32);

impl 位相非対称係数 {
    pub fn 生成する(値: f32) -> Result<Self, 大気数学エラー> {
        if !値.is_finite() || 値 <= -1.0 || 値 >= 1.0 {
            return Err(大気数学エラー::値域外("位相非対称係数", 値));
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> f32 {
        self.0
    }
}
