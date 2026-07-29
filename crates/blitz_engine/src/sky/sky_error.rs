//! 天空状態の導出が拒む入力と、導いた値が値域を外れた事実を表す型付きエラー。

use thiserror::Error;

use crate::time::時刻エラー;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 天空状態エラー {
    #[error("世界時刻から一日内時刻を導けなかった: {0}")]
    時刻を導けない(#[from] 時刻エラー),
    #[error("太陽方向が有限かつゼロ長でない単位ベクトルにならなかった")]
    太陽方向が単位ベクトルでない,
    #[error("{項目}が値域を外れた: {値}")]
    値域外 { 項目: &'static str, 値: f32 },
}

impl 天空状態エラー {
    pub(super) fn 値域外(項目: &'static str, 値: f32) -> Self {
        Self::値域外 { 項目, 値 }
    }
}
