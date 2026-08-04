//! 全体に対する画素の割合。0以上1以下の比であり、0から100の百分率と型で区別する。

use super::{画素件数, 自動露出エラー};

/// 不変条件: 有限かつ0以上1以下である。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 画素の割合(f32);

impl 画素の割合 {
    pub fn 生成する(値: f32) -> Result<Self, 自動露出エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(自動露出エラー::値域外("画素の割合", 値));
        }
        Ok(Self(値))
    }

    /// 部分の件数を全体の件数で割る。全体が0件のときは割合が定まらないため、0を返さず型付きの失敗にする。
    pub fn 件数の比から求める(部分: 画素件数, 全体: 画素件数) -> Result<Self, 自動露出エラー> {
        if 全体.値() == 0 {
            return Err(自動露出エラー::割合の分母が零件);
        }
        Self::生成する(部分.実数で表す() / 全体.実数で表す())
    }

    pub fn 値(&self) -> f32 {
        self.0
    }

    pub fn 百分率(&self) -> f32 {
        self.0 * 100.0
    }
}
