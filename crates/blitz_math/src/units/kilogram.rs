//! 質量の単位型。0以上の有限値だけを持ち、負と非有限は型付きエラーで拒む。

use super::mass_error::{質量の量として検査する, 質量エラー};
use super::逆キログラム;

/// キログラム単位の質量。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct キログラム(f32);

impl キログラム {
    /// 負と非有限を型付きエラーで拒む。0は許す。
    pub fn 生成する(値: f32) -> Result<Self, 質量エラー> {
        質量の量として検査する(値).map(Self)
    }

    /// この質量の逆数。質量0の逆数は無限大になるため、有限値でないとして拒む。
    pub fn 逆質量(&self) -> Result<逆キログラム, 質量エラー> {
        逆キログラム::生成する(1.0 / self.0)
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
    fn 正の質量の逆数は逆質量になる() {
        assert_eq!(キログラム::生成する(4.0).and_then(|質量| 質量.逆質量()), 逆キログラム::生成する(0.25));
    }

    #[test]
    fn 質量0の逆数は拒む() {
        assert_eq!(
            キログラム::生成する(0.0).and_then(|質量| 質量.逆質量()),
            Err(質量エラー::非有限値 { 値: f32::INFINITY })
        );
    }

    #[test]
    fn 負と非数を拒む() {
        assert_eq!(キログラム::生成する(-1.0), Err(質量エラー::負の値 { 値: -1.0 }));
        assert!(matches!(キログラム::生成する(f32::NAN), Err(質量エラー::非有限値 { .. })));
    }
}
