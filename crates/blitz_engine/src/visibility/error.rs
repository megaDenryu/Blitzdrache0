//! 可視判定の材料を組み立てるときの型付きエラー。
//! `影の視距離不正`が拒むのは、0以下が影を落とす個体を1体も持たない状態、非有限が比較の意味を持たない状態になるためである。

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 可視判定エラー {
    個体数過大 { 個体数: usize }, // 添字はu32でGPUへ渡るため、収まらない群は材料にできない
    影の視距離不正, // 影の視距離は正の有限でなければならない
}

impl fmt::Display for 可視判定エラー {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::個体数過大 { 個体数 } => write!(formatter, "群の個体数{個体数}が可視IDのu32に収まらない"),
            Self::影の視距離不正 => write!(formatter, "影の視距離が正の有限でない"),
        }
    }
}

impl std::error::Error for 可視判定エラー {}
