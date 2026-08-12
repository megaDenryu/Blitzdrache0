//! 版付きチャンク目録のファイル1本を指す型。読むと検証済みのチャンク目録になる。
//! 目録はアセットIDだけを保持しパスを持たないため、カタログのような配置先からのパス解決を必要としない。

use std::path::PathBuf;

use super::{chunk_directory::チャンク目録, directory_load_error::実行時チャンク目録読込エラー};
use crate::asset::{実行時形式からチャンク目録を読む, 実行時形式のファイル};

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 実行時チャンク目録のファイル(実行時形式のファイル);

impl 実行時チャンク目録のファイル {
    pub fn パスから作る(パス: PathBuf) -> Self {
        Self(実行時形式のファイル::パスから作る(パス))
    }

    pub fn 読み込む(&self) -> Result<チャンク目録, 実行時チャンク目録読込エラー> {
        Ok(実行時形式からチャンク目録を読む(&self.0.バイト列を読む()?)?)
    }
}
