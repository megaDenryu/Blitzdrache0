//! 安定IDに対応する生成物、ソース依存関係、容量メタデータ。

use std::path::{Path, PathBuf};

use super::asset_metadata::アセットメタデータ;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct カタログ項目 {
    実行時パス: PathBuf,
    ソース依存一覧: Vec<PathBuf>,
    メタデータ: アセットメタデータ,
}

impl カタログ項目 {
    pub fn 生成する(実行時パス: PathBuf, ソース依存一覧: Vec<PathBuf>, メタデータ: アセットメタデータ) -> Self {
        Self {
            実行時パス,
            ソース依存一覧,
            メタデータ,
        }
    }

    pub fn 実行時パス(&self) -> &Path {
        &self.実行時パス
    }

    pub fn ソース依存一覧(&self) -> &[PathBuf] {
        &self.ソース依存一覧
    }

    pub fn メタデータ(&self) -> アセットメタデータ {
        self.メタデータ
    }
}
