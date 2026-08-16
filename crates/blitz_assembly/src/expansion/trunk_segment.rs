//! 幹の節の数え方。節の総数と、下から何番目かを別の型で持つ。
//!
//! どちらも裸の整数で持つと、総数を番号の位置へ渡しても型が通る。木の高さと枝の付く高さは別の量である。
//! **どちらも1から数える。** 根に据わる幹の一節が1節目であり、0節の木は木でないため型で拒む。

use super::error_rule::規則エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 幹の節数(usize);

impl 幹の節数 {
    /// 零を拒む。根の1節は必ず据わるため、節数の下限は1である。
    pub fn 生成する(節数: usize) -> Result<Self, 規則エラー> {
        if 節数 == 0 {
            return Err(規則エラー::幹の節数が零である);
        }
        Ok(Self(節数))
    }

    pub fn 値(self) -> usize {
        self.0
    }
}

/// 下から数えた幹の節の番号。根の節が1である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 幹の節の番号(usize);

impl 幹の節の番号 {
    pub fn 生成する(番号: usize) -> Result<Self, 規則エラー> {
        if 番号 == 0 {
            return Err(規則エラー::幹の節の番号が零である);
        }
        Ok(Self(番号))
    }

    pub fn 値(self) -> usize {
        self.0
    }

    /// その節が、節数の内側に在るか。木より高い位置に枝を生やす規則をここで拒む。
    pub(super) fn 節数の内側か(self, 節数: 幹の節数) -> bool {
        self.0 <= 節数.値()
    }
}
