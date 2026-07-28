//! ジョイント階層を読み、頂点スキン属性との整合を検査する。

use blitz_math::{ローカル, 変換};

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use crate::asset::{joint::ジョイント, skin_data::スキンデータ, skin_vertex_attribute::スキン頂点属性, static_trs::静的TRS};

const ジョイント最小長: usize = 105;

pub(in crate::asset::runtime_format::scene) fn 読む(
    入力: &mut 読取位置<'_>,
) -> Result<Option<スキンデータ>, アセット実行時形式エラー> {
    match 入力.u8()? {
        0 => Ok(None),
        1 => {
            let ジョイント数 = 入力.件数(ジョイント最小長)?;
            if ジョイント数 == 0 {
                return Err(アセット実行時形式エラー::ジョイントなし);
            }
            let mut ジョイント一覧 = Vec::with_capacity(ジョイント数);
            for ジョイント添字 in 0..ジョイント数 {
                ジョイント一覧.push(ジョイントを読む(入力, ジョイント添字)?);
            }
            Ok(Some(スキンデータ { ジョイント一覧 }))
        }
        不正 => Err(アセット実行時形式エラー::不正な有無判別値(不正)),
    }
}

fn ジョイントを読む(
    入力: &mut 読取位置<'_>, ジョイント添字: usize
) -> Result<ジョイント, アセット実行時形式エラー> {
    let 親添字 = match 入力.u8()? {
        0 => None,
        1 => Some(入力.usize()?),
        不正 => return Err(アセット実行時形式エラー::不正な有無判別値(不正)),
    };
    if let Some(親添字) = 親添字
        && 親添字 >= ジョイント添字
    {
        return Err(アセット実行時形式エラー::親添字順序違反 {
            ジョイント添字, 親添字
        });
    }
    let 逆バインド行列 = 変換::<ローカル, ローカル>::列優先配列から生成する(super::matrix::行列を読む(入力)?);
    Ok(ジョイント {
        親添字,
        逆バインド行列,
        バインド時: trsを読む(入力)?,
    })
}

fn trsを読む(入力: &mut 読取位置<'_>) -> Result<静的TRS, アセット実行時形式エラー> {
    Ok(静的TRS {
        平行移動: [入力.f32()?, 入力.f32()?, 入力.f32()?],
        回転: [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
        スケール: [入力.f32()?, 入力.f32()?, 入力.f32()?],
    })
}

/// 描画対象でなく属性一覧そのものを受け取るのは、描画対象の形状が版で変わる一方この検査の内容は変わらないためである。
pub(in crate::asset::runtime_format::scene) fn 頂点属性を検査する<'a>(
    描画対象ごとの属性一覧: impl Iterator<Item = Option<&'a Vec<スキン頂点属性>>>,
    スキン: Option<&スキンデータ>,
) -> Result<(), アセット実行時形式エラー> {
    let ジョイント数 = スキン.map(|値| 値.ジョイント一覧.len());
    for 属性一覧 in 描画対象ごとの属性一覧 {
        let Some(属性一覧) = 属性一覧 else {
            continue;
        };
        let Some(ジョイント数) = ジョイント数 else {
            return Err(アセット実行時形式エラー::スキンなし頂点属性);
        };
        for 属性 in 属性一覧 {
            for &添字 in &属性.ジョイント {
                if usize::from(添字) >= ジョイント数 {
                    return Err(アセット実行時形式エラー::スキンジョイント範囲外 {
                        ジョイント添字: 添字,
                        ジョイント数,
                    });
                }
            }
        }
    }
    Ok(())
}
