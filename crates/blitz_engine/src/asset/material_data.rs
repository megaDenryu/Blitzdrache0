//! シェーディングモデルごとのマテリアルデータ。地表の層の重ね合わせのデータは`surface_layer_blend`が持つ。

mod surface_layer_blend;

pub use surface_layer_blend::{地表の層の重ね合わせデータ, 地表の層の重ね合わせデータエラー};

use super::pbr_material_data::金属粗さPBRデータ;

/// マテリアル種別を型で判別し、単一のシェーディングモデルをデータ形式へ固定しない。
/// 地表の層の重ね合わせを特徴ビットでなく枝にするのは、アルベドの組み立てが相互排他に変わるためである
/// (参照: `_doc/設計/マルチマテリアルと材質境界.md`「地表の層の重ね合わせ」)。
#[derive(Debug, Clone, PartialEq)]
pub enum マテリアルデータ {
    金属粗さPBR(金属粗さPBRデータ),
    地表の層の重ね合わせ(地表の層の重ね合わせデータ),
}
