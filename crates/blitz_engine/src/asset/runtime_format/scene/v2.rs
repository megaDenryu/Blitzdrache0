//! 版2のシーンと、最新版への変換。版2は描画形状として通常メッシュと地形LODメッシュ群の2つだけを判別し、インスタンス群を表せない。
//! 版2専用の型を残して変換で最新版へ写すため、最新版の型に「版2にはこの変種が無い」という分岐が入り込まない。
//! 参照: `_doc/設計/アセット実行時形式.md`「シーン内容の版2」

#[cfg(test)]
mod conversion_tests;
mod read;
#[cfg(test)]
mod write;

use blitz_math::{ローカル, ワールド, 変換};

use crate::asset::animation_clip::アニメーションクリップ;
use crate::asset::draw_shape::描画形状;
use crate::asset::material_data::マテリアルデータ;
use crate::asset::material_set::材質集合;
use crate::asset::mesh_data::メッシュデータ;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;
use crate::asset::skin_data::スキンデータ;
use crate::asset::terrain_lod_meshes::地形LODメッシュ群;
use crate::チャンク座標;

pub(super) use read::シーン内容を読む;

pub(super) struct シーンV2 {
    先頭の描画対象: 描画対象V2,
    残りの描画対象一覧: Vec<描画対象V2>,
    スキン: Option<スキンデータ>,
    アニメーション一覧: Vec<アニメーションクリップ>,
}

pub(super) struct 描画対象V2 {
    識別子: 描画対象ID,
    所有チャンク: チャンク座標,
    ローカルからワールド: 変換<ローカル, ワールド>,
    形状: 形状V2,
    マテリアル: マテリアルデータ,
}

pub(super) enum 形状V2 {
    通常メッシュ(メッシュデータ),
    地形LODメッシュ群(地形LODメッシュ群),
}

impl シーンV2 {
    /// 参照ファイル一覧は読込器がファイルパスを足すため空で作る。
    pub(super) fn 最新へ変換する(self) -> シーンデータ {
        シーンデータ::生成する(
            self.先頭の描画対象.最新へ変換する(),
            self.残りの描画対象一覧.into_iter().map(描画対象V2::最新へ変換する).collect(),
            Vec::new(),
            self.スキン,
            self.アニメーション一覧,
        )
    }
}

impl 描画対象V2 {
    fn 最新へ変換する(self) -> 描画対象データ {
        描画対象データ::生成する(
            self.識別子,
            self.所有チャンク,
            self.ローカルからワールド,
            self.形状.最新へ変換する(),
            材質集合::単一材質から生成する(self.マテリアル),
        )
    }
}

impl 形状V2 {
    fn 最新へ変換する(self) -> 描画形状 {
        match self {
            Self::通常メッシュ(メッシュ) => 描画形状::通常メッシュ(メッシュ),
            Self::地形LODメッシュ群(群) => 描画形状::地形LODメッシュ群(群),
        }
    }

    pub(super) fn スキン頂点属性一覧(&self) -> Option<&Vec<crate::asset::skin_vertex_attribute::スキン頂点属性>> {
        match self {
            Self::通常メッシュ(メッシュ) => メッシュ.スキン頂点属性一覧.as_ref(),
            Self::地形LODメッシュ群(_) => None,
        }
    }
}
