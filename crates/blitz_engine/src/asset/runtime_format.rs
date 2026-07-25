//! アセット実行時形式の共通ヘッダーと検証済みの読み取り結果。

mod catalog_v1;
mod chunk_directory_v1;
mod error;
mod header;
mod scene_v1;

pub use catalog_v1::{カタログを実行時形式へ格納する, 実行時形式からカタログを読む};
pub use chunk_directory_v1::{チャンク目録を実行時形式へ格納する, 実行時形式からチャンク目録を読む};
pub use error::アセット実行時形式エラー;
pub use header::{実行時アセットを格納する, 実行時アセットを開く};
pub(crate) use scene_v1::mesh_layout;
pub use scene_v1::{シーンを実行時形式へ格納する, 実行時形式からシーンを読む};

pub(super) const ヘッダー長: usize = 24;
pub(super) const 固定識別値: [u8; 8] = *b"BLITZAST";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum アセット形式版 {
    V1,
}

impl アセット形式版 {
    pub fn 番号(self) -> u32 {
        match self {
            Self::V1 => 1,
        }
    }

    pub(super) fn 番号から読む(番号: u32) -> Result<Self, アセット実行時形式エラー> {
        match 番号 {
            1 => Ok(Self::V1),
            未対応 => Err(アセット実行時形式エラー::未対応形式版(未対応)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 実行時アセット種別 {
    シーン,
    カタログ,
    チャンク目録,
}

impl 実行時アセット種別 {
    pub fn 番号(self) -> u32 {
        match self {
            Self::シーン => 1,
            Self::カタログ => 2,
            Self::チャンク目録 => 3,
        }
    }

    pub(super) fn 番号から読む(番号: u32) -> Result<Self, アセット実行時形式エラー> {
        match 番号 {
            1 => Ok(Self::シーン),
            2 => Ok(Self::カタログ),
            3 => Ok(Self::チャンク目録),
            未知 => Err(アセット実行時形式エラー::未知のアセット種別(未知)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 実行時アセット<'a> {
    pub 形式版: アセット形式版,
    pub 種別: 実行時アセット種別,
    pub 内容: &'a [u8],
}
