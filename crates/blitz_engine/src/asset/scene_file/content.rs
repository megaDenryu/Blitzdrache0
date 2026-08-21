//! 実行時シーンの格納バイト列を、読み元のファイルと組にして比較・復元できる値として持つ。

use super::{シーンデータ, 実行時シーン読込エラー, 実行時形式からシーンを読む};
use crate::asset::runtime_file::実行時形式のファイル;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 実行時シーンの内容 {
    ファイル: 実行時形式のファイル,
    バイト列: Vec<u8>,
}

impl 実行時シーンの内容 {
    pub(super) fn ファイルから読み込む(ファイル: 実行時形式のファイル) -> Result<Self, 実行時シーン読込エラー> {
        let バイト列 = ファイル.バイト列を読む()?;
        Ok(Self { ファイル, バイト列 })
    }

    pub fn シーンデータへ読む(&self) -> Result<シーンデータ, 実行時シーン読込エラー> {
        let mut シーン = 実行時形式からシーンを読む(&self.バイト列)?;
        シーン.参照ファイル一覧.push(self.ファイル.パスの写しを作る());
        Ok(シーン)
    }

    pub fn 格納バイト列が同じか(&self, 比較先: &Self) -> bool {
        self.バイト列 == 比較先.バイト列
    }

    pub(super) fn バイト数(&self) -> usize {
        self.バイト列.len()
    }
}
