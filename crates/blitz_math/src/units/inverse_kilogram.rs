//! 逆質量の単位型。XPBDの拘束が自由度の重みとして使う量であり、コンプライアンスを刻み幅の2乗で割った量も同じ次元を持つ。
//! 0を許すのは、固定した自由度を解法の内部で重み0として扱うためである(質量の正本を0で表す意味ではない)。

use std::ops::Add;

use super::mass_error::{質量の量として検査する, 質量エラー};

/// 毎キログラム単位の逆質量。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 逆キログラム(f32);

impl 逆キログラム {
    /// 負と非有限を型付きエラーで拒む。0は許す。
    pub fn 生成する(値: f32) -> Result<Self, 質量エラー> {
        質量の量として検査する(値).map(Self)
    }

    /// 動かせない自由度の重み。
    pub fn 零() -> Self {
        Self(0.0)
    }

    pub fn 零か(&self) -> bool {
        self.0 == 0.0
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for 逆キログラム {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 逆質量どうしは足せる() {
        assert_eq!(逆キログラム::生成する(0.5).map(|左| 左 + 逆キログラム::零()), 逆キログラム::生成する(0.5));
    }

    #[test]
    fn 負と非有限を拒む() {
        assert_eq!(逆キログラム::生成する(-0.5), Err(質量エラー::負の値 { 値: -0.5 }));
        assert_eq!(逆キログラム::生成する(f32::INFINITY), Err(質量エラー::非有限値 { 値: f32::INFINITY }));
    }
}
