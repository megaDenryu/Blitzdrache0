//! 版1から版3までの材質1件を読む。材質特徴集合を持たない並びであり、テクスチャの有無だけが材質の機能を決める。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::material_element::{係数を読む, 金属粗さPBRの種別番号};
use super::texture::版4までを読む as テクスチャを読む;
use crate::asset::material_data::マテリアルデータ;
use crate::asset::pbr_material_data::金属粗さPBRデータ;

pub(in crate::asset::runtime_format::scene) fn 読む(
    入力: &mut 読取位置<'_>,
) -> Result<マテリアルデータ, アセット実行時形式エラー> {
    let 種別 = 入力.u32()?;
    if 種別 != 金属粗さPBRの種別番号 {
        return Err(アセット実行時形式エラー::未知のマテリアル種別(種別));
    }
    Ok(マテリアルデータ::金属粗さPBR(金属粗さPBRデータ {
        ベースカラー: テクスチャを読む(入力)?,
        金属粗さ: テクスチャを読む(入力)?,
        法線マップ: テクスチャを読む(入力)?,
        ベースカラー係数: [係数を読む(入力)?, 係数を読む(入力)?, 係数を読む(入力)?, 係数を読む(入力)?],
        金属度係数: 係数を読む(入力)?,
        粗さ係数: 係数を読む(入力)?,
    }))
}
