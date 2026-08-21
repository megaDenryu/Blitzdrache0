//! 生成した建物外形カタログを置く役割つきのパス。配置と原子的な書き出しをこの型へ閉じる。

use std::path::{Path, PathBuf};

use super::catalog::建物外形カタログ;
use super::catalog_json::整形済みのバイト列を作る;
use super::error::建物外形カタログエラー;

#[derive(Debug, Clone)]
pub struct 建物外形カタログのファイル {
    パス: PathBuf,
}

impl 建物外形カタログのファイル {
    pub fn リポジトリルートから生成する(リポジトリルート: &Path) -> Self {
        Self::生成する(リポジトリルート.join("target").join("editor").join("building_outline_catalog.json"))
    }

    pub fn 生成する(パス: PathBuf) -> Self {
        Self { パス }
    }

    pub fn パス(&self) -> &Path {
        &self.パス
    }

    pub fn 書き出す(&self, カタログ: &建物外形カタログ) -> Result<(), 建物外形カタログエラー> {
        if let Some(親) = self.パス.parent() {
            std::fs::create_dir_all(親).map_err(|原因| self.書き込みエラー(原因))?;
        }
        let 内容 = 整形済みのバイト列を作る(カタログ)?;
        crate::atomic_file_write::一時ファイル経由で書き込む(&self.パス, &内容).map_err(|原因| self.書き込みエラー(原因))
    }

    fn 書き込みエラー(&self, 原因: std::io::Error) -> 建物外形カタログエラー {
        建物外形カタログエラー::ファイルを書き込めない {
            パス: self.パス.display().to_string(),
            原因,
        }
    }
}
