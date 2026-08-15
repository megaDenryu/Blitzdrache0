//! 散布が持つ2つの範囲。担当するのは、受け取れる値域を答えることと、範囲の内側の1つの値へ写すことである。
//!
//! 密度場の値も混合関数の値も、そのままでは意味を持たない-1以上1以下と0以上1未満の数でしかない。
//! 意味のある量(出現の割合と大きさの倍率)へ写すのはこの2つの型であり、写し方を呼び出し側へ散らさない。

#[cfg(test)]
mod range_tests;

use crate::error::アセットコンパイルエラー;

/// 密度場の値から出現の割合へ写す範囲。密度場が最も低い所で下限、最も高い所で上限になる。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 出現割合の範囲 {
    pub 下限: f32,
    pub 上限: f32,
}

/// 個体の大きさの倍率の範囲。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 大きさの範囲 {
    pub 下限: f32,
    pub 上限: f32,
}

impl 出現割合の範囲 {
    pub(super) fn 検証する(self) -> Result<(), アセットコンパイルエラー> {
        let 受け取れる = self.下限.is_finite()
            && self.上限.is_finite()
            && (0.0..=1.0).contains(&self.下限)
            && (0.0..=1.0).contains(&self.上限)
            && self.下限 <= self.上限;
        if 受け取れる {
            return Ok(());
        }
        Err(アセットコンパイルエラー::散布の出現割合不正 {
            下限: self.下限,
            上限: self.上限,
        })
    }

    /// -1以上1以下の密度場の値を出現の割合へ写す。
    pub(super) fn 密度場の値から求める(self, 密度場の値: f32) -> f32 {
        self.下限 + (self.上限 - self.下限) * (密度場の値 + 1.0) * 0.5
    }
}

impl 大きさの範囲 {
    pub(super) fn 検証する(self) -> Result<(), アセットコンパイルエラー> {
        if self.下限.is_finite() && self.上限.is_finite() && self.下限 > 0.0 && self.下限 <= self.上限 {
            return Ok(());
        }
        Err(アセットコンパイルエラー::散布の大きさの範囲不正 {
            下限: self.下限,
            上限: self.上限,
        })
    }

    pub(super) fn 零以上一未満の値から求める(self, 比: f32) -> f32 {
        self.下限 + (self.上限 - self.下限) * 比
    }
}
