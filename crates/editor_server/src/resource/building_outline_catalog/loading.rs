//! 建物外形カタログのファイル読み込み境界と型付きエラー。

use std::path::Path;

use thiserror::Error;

use super::建物外形カタログ;

#[derive(Debug, Error)]
pub enum 建物外形カタログ読み込みエラー {
    #[error("建物外形カタログを読めない: {パス}: {原因}")]
    ファイルを読めない {
        パス: String,
        #[source]
        原因: std::io::Error,
    },
    #[error("建物外形カタログのJSONが不正である: {パス}: {原因}")]
    JSONが不正 {
        パス: String,
        #[source]
        原因: serde_json::Error,
    },
    #[error("建物外形カタログの形式版{実際}には対応していない（対応版: {対応}）")]
    未対応の形式版 { 実際: u32, 対応: u32 },
    #[error("建物外形カタログの内容が不正である: {0}")]
    内容が不正(String),
}

impl 建物外形カタログ {
    pub fn ファイルから読み取る(パス: &Path) -> Result<Self, 建物外形カタログ読み込みエラー> {
        let バイト列 = std::fs::read(パス).map_err(|原因| 建物外形カタログ読み込みエラー::ファイルを読めない {
            パス: パス.display().to_string(),
            原因,
        })?;
        let カタログ: Self = serde_json::from_slice(&バイト列).map_err(|原因| 建物外形カタログ読み込みエラー::JSONが不正 {
            パス: パス.display().to_string(),
            原因,
        })?;
        カタログ.検証する()?;
        Ok(カタログ)
    }
}
