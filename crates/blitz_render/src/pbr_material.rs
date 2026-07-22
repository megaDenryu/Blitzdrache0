//! glTFのPBR metallic-roughnessをGPUへ渡す素材。

use thiserror::Error;

use crate::texture_material::テクスチャ素材;

#[derive(Debug, Error, Clone, Copy, PartialEq)]
pub enum マテリアル素材エラー {
    #[error("マテリアル係数が0.0から1.0の範囲外だった: {0}")]
    係数範囲外(f32),
}

#[derive(Debug, Clone)]
pub struct 金属粗さPBR素材 {
    ベースカラー: テクスチャ素材,
    金属粗さ: テクスチャ素材,
    法線マップ: テクスチャ素材,
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
}

impl 金属粗さPBR素材 {
    #[allow(clippy::too_many_arguments)]
    pub fn 生成する(
        ベースカラー: テクスチャ素材,
        金属粗さ: テクスチャ素材,
        法線マップ: テクスチャ素材,
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

    pub(crate) fn ベースカラー(&self) -> &テクスチャ素材 {
        &self.ベースカラー
    }

    pub(crate) fn 金属粗さ(&self) -> &テクスチャ素材 {
        &self.金属粗さ
    }

    pub(crate) fn 法線マップ(&self) -> &テクスチャ素材 {
        &self.法線マップ
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
