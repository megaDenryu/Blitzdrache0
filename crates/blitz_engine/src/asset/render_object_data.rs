//! 描画対象データ: メッシュ、マテリアル、配置、所有チャンクを束ねる。

use blitz_math::{ローカル, ワールド, 変換};

use super::chunk_id::チャンクID;
use super::material_data::マテリアルデータ;
use super::mesh_data::メッシュデータ;
use super::render_object_id::描画対象ID;

/// シーンから描画処理へ抽出できる1つの対象。
#[derive(Debug, Clone, PartialEq)]
pub struct 描画対象データ {
    識別子: 描画対象ID,
    所有チャンク: チャンクID,
    ローカルからワールド: 変換<ローカル, ワールド>,
    メッシュ: メッシュデータ,
    マテリアル: マテリアルデータ,
}

impl 描画対象データ {
    pub fn 生成する(
        識別子: 描画対象ID,
        所有チャンク: チャンクID,
        ローカルからワールド: 変換<ローカル, ワールド>,
        メッシュ: メッシュデータ,
        マテリアル: マテリアルデータ,
    ) -> Self {
        Self {
            識別子,
            所有チャンク,
            ローカルからワールド,
            メッシュ,
            マテリアル,
        }
    }

    pub fn 識別子(&self) -> 描画対象ID {
        self.識別子
    }

    pub fn 所有チャンク(&self) -> チャンクID {
        self.所有チャンク
    }

    pub fn ローカルからワールド(&self) -> 変換<ローカル, ワールド> {
        self.ローカルからワールド
    }

    pub fn メッシュ(&self) -> &メッシュデータ {
        &self.メッシュ
    }

    pub fn マテリアル(&self) -> &マテリアルデータ {
        &self.マテリアル
    }
}
