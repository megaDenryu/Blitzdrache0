//! 版1のシーンと、最新版への変換。版1は描画形状の判別を持たず、すべての描画対象がちょうど1つのメッシュで描かれる。
//! 版1専用の型を残して変換で最新版へ写すため、最新版の型に「版1にはこの欄が無い」という分岐が入り込まない。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「地形の表現」

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
use crate::チャンク座標;

pub(super) use read::シーン内容を読む;

pub(super) struct シーンV1 {
    先頭の描画対象: 描画対象V1,
    残りの描画対象一覧: Vec<描画対象V1>,
    スキン: Option<スキンデータ>,
    アニメーション一覧: Vec<アニメーションクリップ>,
}

pub(super) struct 描画対象V1 {
    識別子: 描画対象ID,
    所有チャンク: チャンク座標,
    ローカルからワールド: 変換<ローカル, ワールド>,
    メッシュ: メッシュデータ,
    マテリアル: マテリアルデータ,
}

impl シーンV1 {
    /// 版1の全描画対象は通常メッシュとして最新版へ移る。参照ファイル一覧は読込器がファイルパスを足すため空で作る。
    pub(super) fn 最新へ変換する(self) -> シーンデータ {
        シーンデータ::生成する(
            self.先頭の描画対象.最新へ変換する(),
            self.残りの描画対象一覧.into_iter().map(描画対象V1::最新へ変換する).collect(),
            Vec::new(),
            self.スキン,
            self.アニメーション一覧,
        )
    }
}

impl 描画対象V1 {
    fn 最新へ変換する(self) -> 描画対象データ {
        描画対象データ::生成する(
            self.識別子,
            self.所有チャンク,
            self.ローカルからワールド,
            描画形状::通常メッシュ(self.メッシュ),
            材質集合::単一材質から生成する(self.マテリアル),
        )
    }
}
