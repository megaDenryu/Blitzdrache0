//! シェーディングモデルごとのマテリアルデータ。

use super::pbr_material_data::金属粗さPBRデータ;

/// マテリアル種別を型で判別し、単一のシェーディングモデルをデータ形式へ固定しない。
#[derive(Debug, Clone, PartialEq)]
pub enum マテリアルデータ {
    金属粗さPBR(金属粗さPBRデータ),
}
