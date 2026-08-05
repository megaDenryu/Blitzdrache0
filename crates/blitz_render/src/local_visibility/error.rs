//! 局所可視性補正の入力が受け付けない値と、要素数の食い違いを表す型付きエラー。
//!
//! 派生表現エラーと別の型にするのは、あちらが遠方環境の畳み込みの入力を、こちらが画面空間の深度と
//! 遮蔽の設定を守るためである。同じ型へ混ぜると、畳み込みの入力の違反と遮蔽の設定の違反が
//! 呼び出し側で見分けられなくなる。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 局所可視性エラー {
    #[error("{項目}が値域を外れた: {値}")]
    値域外 { 項目: &'static str, 値: f32 },
    #[error("{項目}が値域を外れた: {値}")]
    整数値域外 { 項目: &'static str, 値: u32 },
    #[error("{項目}の要素数が{実際}件で、期待する{期待}件と違う")]
    要素数不一致 { 項目: &'static str, 期待: usize, 実際: usize },
    #[error("{項目}の下限{下限}が上限{上限}以上だった")]
    上下の順序が逆 { 項目: &'static str, 下限: f32, 上限: f32 },
}

impl 局所可視性エラー {
    pub(super) fn 値域外(項目: &'static str, 値: f32) -> Self {
        Self::値域外 { 項目, 値 }
    }

    pub(super) fn 整数値域外(項目: &'static str, 値: u32) -> Self {
        Self::整数値域外 { 項目, 値 }
    }

    pub(super) fn 要素数不一致(項目: &'static str, 期待: usize, 実際: usize) -> Self {
        Self::要素数不一致 { 項目, 期待, 実際 }
    }

    pub(super) fn 上下の順序が逆(項目: &'static str, 下限: f32, 上限: f32) -> Self {
        Self::上下の順序が逆 { 項目, 下限, 上限 }
    }
}
