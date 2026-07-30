//! 大気数学の入力が受け付けない値を表す型付きエラー。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 大気数学エラー {
    #[error("{項目}が値域を外れた: {値}")]
    値域外 { 項目: &'static str, 値: f32 },
    #[error("{項目}が値域を外れた: {値}")]
    整数値域外 { 項目: &'static str, 値: u32 },
}

impl 大気数学エラー {
    pub(in crate::atmosphere) fn 値域外(項目: &'static str, 値: f32) -> Self {
        Self::値域外 { 項目, 値 }
    }

    pub(in crate::atmosphere) fn 整数値域外(項目: &'static str, 値: u32) -> Self {
        Self::整数値域外 { 項目, 値 }
    }
}
