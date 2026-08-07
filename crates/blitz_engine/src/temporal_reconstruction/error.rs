//! 時間再構成の純関数層が受け付けない値を表す型付きエラー。
//!
//! 自動露出エラーと別の型にするのは、あちらが輝度の目盛と露出の追従を、こちらが動きベクトルの規約と
//! 履歴の混合の入力を守るためである。同じ型へ混ぜると、露出の値域違反と履歴の値域違反が呼び出し側で
//! 見分けられなくなる。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum 時間再構成エラー {
    #[error("{項目}が値域を外れた: {値}")]
    値域外 { 項目: &'static str, 値: f32 },
    #[error("{項目}が値域を外れた: {値}")]
    整数値域外 { 項目: &'static str, 値: u32 },
    #[error("{項目}の要素が1件も無い")]
    要素が空 { 項目: &'static str },
}

impl 時間再構成エラー {
    pub(super) fn 値域外(項目: &'static str, 値: f32) -> Self {
        Self::値域外 { 項目, 値 }
    }

    pub(super) fn 整数値域外(項目: &'static str, 値: u32) -> Self {
        Self::整数値域外 { 項目, 値 }
    }

    pub(super) fn 要素が空(項目: &'static str) -> Self {
        Self::要素が空 { 項目 }
    }
}
