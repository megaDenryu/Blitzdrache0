//! マテリアルデータの抽出: ベースカラー・metallicRoughness・法線マップ
//! (いずれも無ければNone)と係数(baseColor/metallic/roughness)をまとめる(判断23)。

use std::path::PathBuf;

use crate::asset::error::アセットエラー;
use crate::asset::material_data::マテリアルデータ;
use crate::asset::pbr_material_data::金属粗さPBRデータ;
use crate::asset::texture_data::テクスチャデータ;

use super::document::開いた文書;
use super::texture_decode;

pub(super) fn マテリアルを取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
) -> Result<(マテリアルデータ, Vec<PathBuf>), アセットエラー> {
    let マテリアル = プリミティブ.material();
    let pbr = マテリアル.pbr_metallic_roughness();
    let mut 参照ファイル一覧 = Vec::new();

    let ベースカラー = 情報から取り出す(文書, pbr.base_color_texture(), &mut 参照ファイル一覧)?;
    let 金属粗さ = 情報から取り出す(文書, pbr.metallic_roughness_texture(), &mut 参照ファイル一覧)?;
    let 法線マップ = 法線情報から取り出す(文書, マテリアル.normal_texture(), &mut 参照ファイル一覧)?;

    Ok((
        マテリアルデータ::金属粗さPBR(金属粗さPBRデータ {
            ベースカラー,
            金属粗さ,
            法線マップ,
            ベースカラー係数: pbr.base_color_factor(),
            金属度係数: pbr.metallic_factor(),
            粗さ係数: pbr.roughness_factor(),
        }),
        参照ファイル一覧,
    ))
}

fn 情報から取り出す(
    文書: &開いた文書,
    情報: Option<gltf::texture::Info<'_>>,
    参照ファイル一覧: &mut Vec<PathBuf>,
) -> Result<Option<テクスチャデータ>, アセットエラー> {
    let Some(情報) = 情報 else {
        return Ok(None);
    };
    let (データ, パス) = texture_decode::デコードする(文書, &情報.texture())?;
    if let Some(パス) = パス {
        参照ファイル一覧.push(パス);
    }
    Ok(Some(データ))
}

fn 法線情報から取り出す(
    文書: &開いた文書,
    情報: Option<gltf::material::NormalTexture<'_>>,
    参照ファイル一覧: &mut Vec<PathBuf>,
) -> Result<Option<テクスチャデータ>, アセットエラー> {
    let Some(情報) = 情報 else {
        return Ok(None);
    };
    let (データ, パス) = texture_decode::デコードする(文書, &情報.texture())?;
    if let Some(パス) = パス {
        参照ファイル一覧.push(パス);
    }
    Ok(Some(データ))
}
