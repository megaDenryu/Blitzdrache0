//! 符号を持つ逆質量の単位型。接触点集合の法線を同時に解く有効逆質量行列の成分がこの量である。
//! 対角の成分は逆質量そのものであり0以上だが、非対角の成分 (r_i × n_i)ᵀ I⁻¹ (r_j × n_j) は、2つの接触点が
//! 剛体へ逆向きの回転を要求するときに負になる。`逆キログラム` が負を拒むため、行列の成分はこの型で持つ。

use std::ops::Add;

use super::inverse_kilogram::逆キログラム;
use super::mass_error::質量エラー;

/// 毎キログラム単位の符号を持つ量。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 符号付き逆キログラム(f32);

impl 符号付き逆キログラム {
    /// 非有限を型付きエラーで拒む。負は許す。
    pub fn 生成する(値: f32) -> Result<Self, 質量エラー> {
        if 値.is_finite() {
            Ok(Self(値))
        } else {
            Err(質量エラー::非有限値 { 値 })
        }
    }

    pub fn 零() -> Self {
        Self(0.0)
    }

    /// 0以上と分かっている逆質量を符号を持つ量として読む。行列の対角の成分がこの向きの変換で入る。
    pub fn 逆質量から生成する(逆質量: 逆キログラム) -> Self {
        Self(逆質量.値())
    }

    /// 無次元の比で伸ばした量。2つの接触法線の内積を掛けるときに使う。
    pub fn 比で伸ばす(self, 比: f32) -> Self {
        Self(self.0 * 比)
    }

    /// 生値取り出し。この量を内側へ閉じる有効逆質量行列の生成と、GPU・外部APIの境界でだけ使う。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for 符号付き逆キログラム {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}
