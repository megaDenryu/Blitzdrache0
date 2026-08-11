//! 高さ格子のファイル1本を表す型。パスを1回だけ持ち、揃っているかの確認・内容ハッシュの算出・書き出しの3つを
//! 同じパスの上で行う。
//!
//! チャンク1つにつきこの型を1回だけ作るのは、同じパスを操作のたびに組み直さないためである。
//! 3つの操作はどれもチャンク1つの焼き直しの判定と書き出しで続けて呼ばれる。

use std::path::PathBuf;

use blitz_asset_compiler::内容ハッシュ;

#[repr(transparent)]
pub(crate) struct 高さ格子のファイル(PathBuf);

impl 高さ格子のファイル {
    pub(super) fn パスから作る(パス: PathBuf) -> Self {
        Self(パス)
    }

    pub(crate) fn 揃っているか(&self) -> bool {
        self.0.is_file()
    }

    pub(crate) fn 内容ハッシュを求める(&self) -> Result<Option<内容ハッシュ>, String> {
        内容ハッシュ::ファイルがあれば中身から求める(&self.0).map_err(|誤り| 誤り.to_string())
    }

    pub(crate) fn 書き出す(&self, バイト列: &[u8]) -> Result<(), String> {
        std::fs::write(&self.0, バイト列).map_err(|誤り| format!("{}: {誤り}", self.0.display()))
    }
}
