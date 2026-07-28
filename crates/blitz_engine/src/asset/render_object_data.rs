//! 描画対象データ: 形状、マテリアル、配置、所有チャンクを束ねる。

use blitz_math::{ローカル, ワールド, 変換};

use super::draw_shape::描画形状;
use super::material_data::マテリアルデータ;
use super::render_object_id::描画対象ID;
use crate::チャンク座標;

/// シーンから描画処理へ抽出できる1つの対象。
#[derive(Debug, Clone, PartialEq)]
pub struct 描画対象データ {
    識別子: 描画対象ID,
    所有チャンク: チャンク座標,
    ローカルからワールド: 変換<ローカル, ワールド>,
    形状: 描画形状,
    マテリアル: マテリアルデータ,
}

impl 描画対象データ {
    pub fn 生成する(
        識別子: 描画対象ID,
        所有チャンク: チャンク座標,
        ローカルからワールド: 変換<ローカル, ワールド>,
        形状: 描画形状,
        マテリアル: マテリアルデータ,
    ) -> Self {
        Self {
            識別子,
            所有チャンク,
            ローカルからワールド,
            形状,
            マテリアル,
        }
    }

    pub fn 識別子(&self) -> 描画対象ID {
        self.識別子
    }

    pub fn 所有チャンク(&self) -> チャンク座標 {
        self.所有チャンク
    }

    pub fn ローカルからワールド(&self) -> 変換<ローカル, ワールド> {
        self.ローカルからワールド
    }

    pub fn 形状(&self) -> &描画形状 {
        &self.形状
    }

    pub fn マテリアル(&self) -> &マテリアルデータ {
        &self.マテリアル
    }
}
