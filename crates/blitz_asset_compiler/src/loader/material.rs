//! 金属粗さPBRデータの抽出: ベースカラー・metallicRoughness・法線マップ
//! (いずれも無ければNone)と係数(baseColor/metallic/roughness)をまとめる(判断23)。
//! テクスチャは復号した原寸を、テクスチャ格納方針と材質テクスチャ役割の組が決める格納形式へ焼いてから持つ。
//! 返すのが`マテリアルデータ`でなく金属粗さPBRの枝そのものなのは、glTFが表せるシェーディングモデルがこれ1つだからである。
//! 判別共用体で返すと、呼び出し側にglTFからは決して出てこない枝の手当てを強いることになる。

use std::path::PathBuf;

use blitz_engine::texture_storage::格納済みテクスチャ;
use blitz_engine::金属粗さPBRデータ;

use crate::error::アセットコンパイルエラー;
use crate::texture_storage::{
    テクスチャ格納方針, 方針と役割に従って原寸を格納済みテクスチャへ焼く, 材質テクスチャ役割
};

use super::document::開いた文書;
use super::texture_decode;

pub(super) fn マテリアルを取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
    方針: テクスチャ格納方針,
) -> Result<(金属粗さPBRデータ, Vec<PathBuf>), アセットコンパイルエラー> {
    let マテリアル = プリミティブ.material();
    let pbr = マテリアル.pbr_metallic_roughness();
    let mut 参照ファイル一覧 = Vec::new();

    let ベースカラー = 情報から取り出す(
        文書,
        pbr.base_color_texture(),
        &mut 参照ファイル一覧,
        方針,
        材質テクスチャ役割::ベースカラー,
    )?;
    let 金属粗さ = 情報から取り出す(
        文書,
        pbr.metallic_roughness_texture(),
        &mut 参照ファイル一覧,
        方針,
        材質テクスチャ役割::金属粗さ,
    )?;
    let 法線マップ = 法線情報から取り出す(文書, マテリアル.normal_texture(), &mut 参照ファイル一覧, 方針)?;

    Ok((
        金属粗さPBRデータ {
            ベースカラー,
            金属粗さ,
            法線マップ,
            ベースカラー係数: pbr.base_color_factor(),
            金属度係数: pbr.metallic_factor(),
            粗さ係数: pbr.roughness_factor(),
        },
        参照ファイル一覧,
    ))
}

fn 情報から取り出す(
    文書: &開いた文書,
    情報: Option<gltf::texture::Info<'_>>,
    参照ファイル一覧: &mut Vec<PathBuf>,
    方針: テクスチャ格納方針,
    役割: 材質テクスチャ役割,
) -> Result<Option<格納済みテクスチャ>, アセットコンパイルエラー> {
    let Some(情報) = 情報 else {
        return Ok(None);
    };
    焼いて参照を足す(文書, &情報.texture(), 参照ファイル一覧, 方針, 役割).map(Some)
}

fn 法線情報から取り出す(
    文書: &開いた文書,
    情報: Option<gltf::material::NormalTexture<'_>>,
    参照ファイル一覧: &mut Vec<PathBuf>,
    方針: テクスチャ格納方針,
) -> Result<Option<格納済みテクスチャ>, アセットコンパイルエラー> {
    let Some(情報) = 情報 else {
        return Ok(None);
    };
    焼いて参照を足す(文書, &情報.texture(), 参照ファイル一覧, 方針, 材質テクスチャ役割::法線マップ).map(Some)
}

fn 焼いて参照を足す(
    文書: &開いた文書,
    テクスチャ: &gltf::Texture<'_>,
    参照ファイル一覧: &mut Vec<PathBuf>,
    方針: テクスチャ格納方針,
    役割: 材質テクスチャ役割,
) -> Result<格納済みテクスチャ, アセットコンパイルエラー> {
    let (原寸, パス) = texture_decode::デコードする(文書, テクスチャ)?;
    if let Some(パス) = パス {
        参照ファイル一覧.push(パス);
    }
    方針と役割に従って原寸を格納済みテクスチャへ焼く(方針, 役割, &原寸)
}
