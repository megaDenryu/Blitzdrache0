//! XPBDの参照計算が受け取る1本の物理刻みの長さ。正の有限値だけを持つ。
//! 秒をそのまま受けないのは、コンプライアンスを刻み幅の2乗で割る式が0の刻み幅で非数を作るためであり、その検査を生成の1箇所へ閉じる。

use blitz_math::秒;
use thiserror::Error;

/// 1本の物理刻みの長さ。生成が正の有限値であることを保証する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 刻み幅(秒);

impl 刻み幅 {
    /// 0以下と非有限を型付きエラーで拒む。
    pub fn 生成する(長さ: 秒) -> Result<Self, 刻み幅エラー> {
        if !(長さ.値().is_finite() && 長さ.値() > 0.0) {
            return Err(刻み幅エラー::正の有限値でない { 長さ });
        }
        Ok(Self(長さ))
    }

    pub fn 秒(&self) -> 秒 {
        self.0
    }
}

/// 刻み幅の生成が拒んだ入力。
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 刻み幅エラー {
    #[error("刻み幅は正の有限値でなければならない(指定値: {長さ:?})")]
    正の有限値でない { 長さ: 秒 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正の刻み幅だけを通す() {
        assert!(刻み幅::生成する(秒::生成する(1.0 / 60.0)).is_ok());
        assert!(刻み幅::生成する(秒::生成する(0.0)).is_err());
        assert!(刻み幅::生成する(秒::生成する(-1.0)).is_err());
        assert!(刻み幅::生成する(秒::生成する(f32::NAN)).is_err());
    }
}
