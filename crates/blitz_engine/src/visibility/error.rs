//! 可視判定の材料を組み立てるときの型付きエラー。

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 可視判定エラー {
    /// 個体の添字は可視ID列としてu32でGPUへ渡るため、u32に収まらない群は材料にできない。
    個体数過大 { 個体数: usize },
    /// 影の視距離は正の有限でなければならない。0以下は影を落とす個体が1体も無い状態、非有限は比較が意味を持たない状態になる。
    影の視距離不正,
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
