//! シェーディングモデルごとのマテリアル素材。

use crate::pbr_material::金属粗さPBR素材;
use crate::texture_material::テクスチャ素材;

/// マテリアル種別を型で判別し、新しい方式の追加時に全使用箇所を網羅検査する。
#[derive(Debug, Clone)]
pub enum マテリアル素材 {
    金属粗さPBR(金属粗さPBR素材),
}

impl マテリアル素材 {
    fn 金属粗さpbr(&self) -> &金属粗さPBR素材 {
        match self {
            Self::金属粗さPBR(素材) => 素材,
        }
    }

    pub(crate) fn ベースカラー(&self) -> &テクスチャ素材 {
        self.金属粗さpbr().ベースカラー()
    }

    pub(crate) fn 金属粗さ(&self) -> &テクスチャ素材 {
        self.金属粗さpbr().金属粗さ()
    }

    pub(crate) fn 法線マップ(&self) -> &テクスチャ素材 {
        self.金属粗さpbr().法線マップ()
    }

    pub(crate) fn ベースカラー係数(&self) -> [f32; 4] {
        self.金属粗さpbr().ベースカラー係数()
    }

    pub(crate) fn 金属度係数(&self) -> f32 {
        self.金属粗さpbr().金属度係数()
    }

    pub(crate) fn 粗さ係数(&self) -> f32 {
        self.金属粗さpbr().粗さ係数()
    }
}
