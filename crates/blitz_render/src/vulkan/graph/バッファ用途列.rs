//! パスが宣言するバッファ用途の保持方式。布の一覧は2件分を自身の中に持ち、件数が未確定のパスは可変長一覧を持つ。

use arrayvec::ArrayVec;
use std::ops::Deref;

use super::handle::バッファハンドル;
use super::usage::バッファ用途;

pub(crate) enum バッファ用途列 {
    二件まで(ArrayVec<(バッファハンドル, バッファ用途), 2>),
    任意件数(Vec<(バッファハンドル, バッファ用途)>),
}

impl From<ArrayVec<(バッファハンドル, バッファ用途), 2>> for バッファ用途列 {
    fn from(一覧: ArrayVec<(バッファハンドル, バッファ用途), 2>) -> Self {
        Self::二件まで(一覧)
    }
}

impl From<Vec<(バッファハンドル, バッファ用途)>> for バッファ用途列 {
    fn from(一覧: Vec<(バッファハンドル, バッファ用途)>) -> Self {
        Self::任意件数(一覧)
    }
}

impl Deref for バッファ用途列 {
    type Target = [(バッファハンドル, バッファ用途)];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::二件まで(一覧) => 一覧,
            Self::任意件数(一覧) => 一覧,
        }
    }
}

#[cfg(test)]
#[path = "バッファ用途列/検証.rs"]
mod 検証;
