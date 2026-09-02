//! 細分数: 基本刻みを分割する整数 n(判断19)。0を生成で拒む。細分1本の刻み幅 h = H ÷ n は呼び出し側(コンポジションルート)が`刻み幅`として作る。

use super::error::剛体の参照計算エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 細分数(u32);

impl 細分数 {
    pub fn 生成する(値: u32) -> Result<Self, 剛体の参照計算エラー> {
        if 値 == 0 {
            return Err(剛体の参照計算エラー::細分数が零);
        }
        Ok(Self(値))
    }

    pub fn 値(&self) -> u32 {
        self.0
    }
}
