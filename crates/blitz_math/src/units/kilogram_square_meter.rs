//! 慣性の単位型。剛体の主慣性の成分と、回転だけに作用するXPBD拘束のラグランジュ乗数がこの次元を持つ。
//! 乗数は符号が任意であるため、この型そのものは検証を持たない。主慣性が正であることは剛体の側の型が生成で検査する。

use super::inverse_kilogram_square_meter::逆キログラム平方メートル;
use super::mass_error::質量エラー;

/// キログラム平方メートル単位の慣性の量。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct キログラム平方メートル(f32);

impl キログラム平方メートル {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// この慣性の逆数。慣性0の逆数は無限大になり、負の慣性は負の逆慣性になるため、どちらも型付きエラーで拒む。
    pub fn 逆慣性(&self) -> Result<逆キログラム平方メートル, 質量エラー> {
        逆キログラム平方メートル::生成する(1.0 / self.0)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正の慣性の逆数は逆慣性になり零と負は拒む() {
        assert_eq!(キログラム平方メートル::生成する(4.0).逆慣性(), 逆キログラム平方メートル::生成する(0.25));
        assert_eq!(
            キログラム平方メートル::生成する(0.0).逆慣性(),
            Err(質量エラー::非有限値 { 値: f32::INFINITY })
        );
        assert_eq!(キログラム平方メートル::生成する(-2.0).逆慣性(), Err(質量エラー::負の値 { 値: -0.5 }));
    }
}
