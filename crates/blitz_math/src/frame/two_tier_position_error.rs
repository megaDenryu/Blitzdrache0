//! 二段の位置の生成の失敗。どちらの段が非有限であったかを枝で言う。
//! blitz_mathはglamのみに依存する方針のためthiserrorを使わず`std::error::Error`を手動実装する(様式は`direction_error.rs`に倣う)。

use std::fmt;

/// 二段の位置の生成が拒んだ入力。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum 二段の位置エラー {
    粗い位置が非有限,
    細かい位置が非有限,
}

impl fmt::Display for 二段の位置エラー {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::粗い位置が非有限 => write!(f, "二段の位置の粗い位置に有限でない成分がある"),
            Self::細かい位置が非有限 => write!(f, "二段の位置の細かい位置に有限でない成分がある"),
        }
    }
}

impl std::error::Error for 二段の位置エラー {}
