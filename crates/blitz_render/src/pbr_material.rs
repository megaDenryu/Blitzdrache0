//! glTFのPBR metallic-roughnessをGPUへ渡す素材。
//! 3つのテクスチャを`Option`で持つのは、「無し」を既定の画素で埋めた素材へ無言で置き換えないためである。
//! 無しの材質は特徴ビットが下り、材質テクスチャ表の正準フォールバックのスロットを指す
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。

use thiserror::Error;

use crate::texture_material::テクスチャ素材;
use crate::vulkan::material_table::材質テクスチャ役割;

#[derive(Debug, Error, Clone, Copy, PartialEq)]
pub enum マテリアル素材エラー {
    #[error("マテリアル係数が0.0から1.0の範囲外だった: {0}")]
    係数範囲外(f32),
    #[error("地表の層のタイル倍率が正の有限値でなかった: {0}")]
    タイル倍率が正の有限値でない(f32),
}

#[derive(Debug, Clone)]
pub struct 金属粗さPBR素材 {
    ベースカラー: Option<テクスチャ素材>,
    金属粗さ: Option<テクスチャ素材>,
    法線マップ: Option<テクスチャ素材>,
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
}

impl 金属粗さPBR素材 {
    #[allow(clippy::too_many_arguments)]
    pub fn 生成する(
        ベースカラー: Option<テクスチャ素材>,
        金属粗さ: Option<テクスチャ素材>,
        法線マップ: Option<テクスチャ素材>,
        ベースカラー係数: [f32; 4],
        金属度係数: f32,
        粗さ係数: f32,
    ) -> Result<Self, マテリアル素材エラー> {
        for 成分 in ベースカラー係数.into_iter().chain([金属度係数, 粗さ係数]) {
            if !(0.0..=1.0).contains(&成分) {
                return Err(マテリアル素材エラー::係数範囲外(成分));
            }
        }
        Ok(Self {
            ベースカラー,
            金属粗さ,
            法線マップ,
            ベースカラー係数,
            金属度係数,
            粗さ係数,
        })
    }

    /// 役割で1枚を引く。この枝が持たない役割が`None`になるのは、その材質がその役割のテクスチャを持たないという意味そのものである。
    pub(crate) fn 役割のテクスチャ(&self, 役割: 材質テクスチャ役割) -> Option<&テクスチャ素材> {
        match 役割 {
            材質テクスチャ役割::ベースカラー => self.ベースカラー.as_ref(),
            材質テクスチャ役割::金属粗さ => self.金属粗さ.as_ref(),
            材質テクスチャ役割::法線マップ => self.法線マップ.as_ref(),
            材質テクスチャ役割::地表の層の重み
            | 材質テクスチャ役割::地表の層0のタイル
            | 材質テクスチャ役割::地表の層1のタイル
            | 材質テクスチャ役割::地表の層2のタイル
            | 材質テクスチャ役割::地表の層3のタイル => None,
        }
    }

    pub(crate) fn ベースカラー係数(&self) -> [f32; 4] {
        self.ベースカラー係数
    }

    pub(crate) fn 金属度係数(&self) -> f32 {
        self.金属度係数
    }

    pub(crate) fn 粗さ係数(&self) -> f32 {
        self.粗さ係数
    }
}
