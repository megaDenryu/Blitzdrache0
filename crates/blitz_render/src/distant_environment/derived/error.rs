//! 遠方環境の派生表現の入力が受け付けない値を表す型付きエラー。
//!
//! 大気数学エラーと別の型にするのは、派生表現が大気の数学ではなく遠方環境の畳み込みだからである。
//! 同じ型へ混ぜると、大気の値域の違反と畳み込みの入力の違反が呼び出し側で見分けられなくなる。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 派生表現エラー {
    #[error("{項目}が値域を外れた: {値}")]
    値域外 { 項目: &'static str, 値: f32 },
    #[error("{項目}が値域を外れた: {値}")]
    整数値域外 { 項目: &'static str, 値: u32 },
    #[error("{項目}の要素数が{実際}件で、期待する{期待}件と違う")]
    要素数不一致 { 項目: &'static str, 期待: usize, 実際: usize },
}

impl 派生表現エラー {
    pub(crate) fn 値域外(項目: &'static str, 値: f32) -> Self {
        Self::値域外 { 項目, 値 }
    }

    pub(crate) fn 整数値域外(項目: &'static str, 値: u32) -> Self {
        Self::整数値域外 { 項目, 値 }
    }

    pub(crate) fn 要素数不一致(項目: &'static str, 期待: usize, 実際: usize) -> Self {
        Self::要素数不一致 { 項目, 期待, 実際 }
    }
}
