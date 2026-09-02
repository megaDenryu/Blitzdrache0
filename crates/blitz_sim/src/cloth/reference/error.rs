//! 布の参照計算が拒む入力の包み。

use thiserror::Error;

use crate::xpbd::コンプライアンスエラー;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 布の参照計算エラー {
    #[error("拘束の係数を導けない: {0}")]
    係数を導けない(#[from] コンプライアンスエラー),
}
