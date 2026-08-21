//! 公開完了印である生成台帳の内容を、時刻の分解能に依存せず比較する値として持つ。

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct 生成台帳の公開内容(Vec<u8>);

impl 生成台帳の公開内容 {
    pub(super) fn ファイルから読む(パス: PathBuf) -> Result<Self, std::io::Error> {
        std::fs::read(パス).map(Self)
    }
}
