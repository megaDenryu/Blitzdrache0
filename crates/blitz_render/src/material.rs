//! シェーディングモデルごとのマテリアル素材。

use crate::pbr_material::金属粗さPBR素材;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::material_table::材質テクスチャ役割;

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

    /// 役割で1枚を引く。役割から名前を選ぶ対応をここ1箇所に置くことで、材質レコードの並びと標本の並びがずれない。
    pub(crate) fn 役割のテクスチャ(&self, 役割: 材質テクスチャ役割) -> Option<&テクスチャ素材> {
        let 素材 = self.金属粗さpbr();
        match 役割 {
            材質テクスチャ役割::ベースカラー => 素材.ベースカラー(),
            材質テクスチャ役割::金属粗さ => 素材.金属粗さ(),
            材質テクスチャ役割::法線マップ => 素材.法線マップ(),
        }
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
