//! 版3のシーンと、最新版への変換。版3は形状の判別へインスタンス群を持つが、メッシュのプリミティブ列と描画対象の材質集合を持たない。
//! 版3専用の型を残して変換で最新版へ写すため、最新版の型に「版3にはこの欄が無い」という分岐が入り込まない。
//! メッシュは読み取りの時点で全インデックスを材質スロット0で塗る単一プリミティブへ写るため、この型が版で違えるのは材質の持ち方だけである。
//! 参照: `_doc/設計/アセット実行時形式.md`「シーン内容の版3」

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
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;
use crate::asset::skin_data::スキンデータ;
use crate::チャンク座標;

pub(super) use read::シーン内容を読む;

pub(super) struct シーンV3 {
    先頭の描画対象: 描画対象V3,
    残りの描画対象一覧: Vec<描画対象V3>,
    スキン: Option<スキンデータ>,
    アニメーション一覧: Vec<アニメーションクリップ>,
}

pub(super) struct 描画対象V3 {
    識別子: 描画対象ID,
    所有チャンク: チャンク座標,
    ローカルからワールド: 変換<ローカル, ワールド>,
    形状: 描画形状,
    マテリアル: マテリアルデータ,
}

impl シーンV3 {
    /// 参照ファイル一覧は読込器がファイルパスを足すため空で作る。
    pub(super) fn 最新へ変換する(self) -> シーンデータ {
        シーンデータ::生成する(
            self.先頭の描画対象.最新へ変換する(),
            self.残りの描画対象一覧.into_iter().map(描画対象V3::最新へ変換する).collect(),
            Vec::new(),
            self.スキン,
            self.アニメーション一覧,
        )
    }
}

impl 描画対象V3 {
    /// 版3の単一マテリアルは、要素1件でスロット番号0の材質集合になる。
    fn 最新へ変換する(self) -> 描画対象データ {
        描画対象データ::生成する(
            self.識別子,
            self.所有チャンク,
            self.ローカルからワールド,
            self.形状,
            材質集合::単一材質から生成する(self.マテリアル),
        )
    }
}
