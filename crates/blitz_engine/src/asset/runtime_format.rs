//! アセット実行時形式の共通ヘッダーと検証済みの読み取り結果。

mod catalog;
mod chunk_directory_v1;
mod error;
mod header;
mod height_field_error;
mod height_field_v1;
mod material_assignment_error;
mod scene;

pub use catalog::{カタログを実行時形式へ格納する, 実行時形式からカタログを読む};
pub use chunk_directory_v1::{チャンク目録を実行時形式へ格納する, 実行時形式からチャンク目録を読む};
pub use error::アセット実行時形式エラー;
pub use header::{実行時アセットを格納する, 実行時アセットを開く};
pub use height_field_error::高さ場実行時形式エラー;
pub use height_field_v1::{実行時形式から高さ場を読む, 高さ場を実行時形式へ格納する};
pub use material_assignment_error::材質割当エラー;
pub(crate) use scene::mesh_layout;
pub use scene::{シーンを実行時形式へ格納する, 実行時形式からシーンを読む};

pub(super) const ヘッダー長: usize = 24;
pub(super) const 固定識別値: [u8; 8] = *b"BLITZAST";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum アセット形式版 {
    V1,
    /// シーンでは描画対象が形状の判別値を持つ版、カタログでは容量メタデータへ個体数を加えた版である。
    V2,
    /// シーンでは形状の判別へインスタンス群を加えた版、カタログでは容量メタデータの意味をテクスチャ格納バイト数へ変えた版である。
    V3,
    /// シーンではメッシュへプリミティブ列と材質集合を、カタログでは世界の由来と高さ場標本数を加えた版である。
    V4,
    /// テクスチャへ格納形式と縮小段の列を加えた版。シーンだけがこの版を使う。
    V5,
}

impl アセット形式版 {
    pub fn 番号(self) -> u32 {
        match self {
            Self::V1 => 1,
            Self::V2 => 2,
            Self::V3 => 3,
            Self::V4 => 4,
            Self::V5 => 5,
        }
    }

    pub(super) fn 番号から読む(番号: u32) -> Result<Self, アセット実行時形式エラー> {
        match 番号 {
            1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            3 => Ok(Self::V3),
            4 => Ok(Self::V4),
            5 => Ok(Self::V5),
            未対応 => Err(アセット実行時形式エラー::未対応形式版(未対応)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 実行時アセット種別 {
    シーン,
    カタログ,
    チャンク目録,
    /// 世界の高さを実行中に参照するための種別。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断7: 地図の正本を持ち、生成は2系統に分ける」
    高さ場,
}

impl 実行時アセット種別 {
    pub fn 番号(self) -> u32 {
        match self {
            Self::シーン => 1,
            Self::カタログ => 2,
            Self::チャンク目録 => 3,
            Self::高さ場 => 4,
        }
    }

    pub(super) fn 番号から読む(番号: u32) -> Result<Self, アセット実行時形式エラー> {
        match 番号 {
            1 => Ok(Self::シーン),
            2 => Ok(Self::カタログ),
            3 => Ok(Self::チャンク目録),
            4 => Ok(Self::高さ場),
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
