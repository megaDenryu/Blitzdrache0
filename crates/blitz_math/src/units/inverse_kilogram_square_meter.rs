//! 逆慣性の単位型。角の拘束(曲げ)に参加する点自由度の逆質量に勾配の2乗(毎平方メートル)を掛けた量がこの次元を持ち、
//! 曲げのコンプライアンスを刻み幅の2乗で割った刻み依存量も同じ次元を持つ。逆質量(毎キログラム)と同じく、0は動かせない自由度の重みとして許す。

use std::ops::Add;

use super::mass_error::{質量の量として検査する, 質量エラー};

/// 毎キログラム平方メートル単位の逆慣性。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 逆キログラム平方メートル(f32);

impl 逆キログラム平方メートル {
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

impl Add for 逆キログラム平方メートル {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 逆慣性どうしは足せて負と非有限は拒む() {
        assert_eq!(
            逆キログラム平方メートル::生成する(0.5).map(|左| 左 + 逆キログラム平方メートル::零()),
            逆キログラム平方メートル::生成する(0.5)
        );
        assert!(逆キログラム平方メートル::零().零か());
        assert_eq!(逆キログラム平方メートル::生成する(-0.5), Err(質量エラー::負の値 { 値: -0.5 }));
        assert!(逆キログラム平方メートル::生成する(f32::NAN).is_err());
    }
}
