//! 可視判定の材料を組み立てるときの型付きエラー。

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 可視判定エラー {
    /// 個体の添字は可視ID列としてu32でGPUへ渡るため、u32に収まらない群は材料にできない。
    個体数過大 { 個体数: usize },
}

impl fmt::Display for 可視判定エラー {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::個体数過大 { 個体数 } => write!(formatter, "群の個体数{個体数}が可視IDのu32に収まらない"),
        }
    }
}

impl std::error::Error for 可視判定エラー {}
