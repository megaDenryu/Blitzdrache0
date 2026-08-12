//! 実行時形式のファイルを読み出せなかったことを表す型。どのファイルが、どういう事由で読めなかったかを持つ。
//!
//! 種別ごとの読込エラー(カタログ・シーン・チャンク目録・高さ場)がこの1つを枝で内包するのは、
//! 「ファイルが読めなかった」という同じ破れを4つの綴りで書き分けないためである。

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[error("{}: {事由}", パス.display())]
pub struct 実行時形式のファイルの読み取りの破れ {
    パス: PathBuf,
    事由: std::io::Error,
}

impl 実行時形式のファイルの読み取りの破れ {
    pub(super) fn 生成する(パス: PathBuf, 事由: std::io::Error) -> Self {
        Self { パス, 事由 }
    }
}
