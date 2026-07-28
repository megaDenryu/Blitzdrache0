//! 版1の静的メッシュを検証して読む。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::super::mesh_layout::{インデックス長, スキン頂点属性長, 頂点長};
use crate::asset::mesh_data::メッシュデータ;
use crate::asset::skin_vertex_attribute::スキン頂点属性;
use crate::asset::vertex_attribute::メッシュ頂点属性;

pub(in crate::asset::runtime_format::scene) fn 読む(
    入力: &mut 読取位置<'_>,
) -> Result<メッシュデータ, アセット実行時形式エラー> {
    let 頂点数 = 入力.件数(頂点長)?;
    if 頂点数 == 0 {
        return Err(アセット実行時形式エラー::頂点なし);
    }
    let mut 頂点一覧 = Vec::with_capacity(頂点数);
    for _ in 0..頂点数 {
        頂点一覧.push(頂点を読む(入力)?);
    }
    let 添字数 = 入力.件数(インデックス長)?;
    if 添字数 == 0 {
        return Err(アセット実行時形式エラー::インデックスなし);
    }
    let mut インデックス一覧 = Vec::with_capacity(添字数);
    for _ in 0..添字数 {
        let インデックス = 入力.u32()?;
        if usize::try_from(インデックス).map_or(true, |値| 値 >= 頂点数) {
            return Err(アセット実行時形式エラー::インデックス範囲外 {
                インデックス, 頂点数
            });
        }
        インデックス一覧.push(インデックス);
    }
    let スキン頂点属性一覧 = スキン頂点属性を読む(入力, 頂点数)?;
    Ok(メッシュデータ {
        頂点一覧,
        インデックス一覧,
        スキン頂点属性一覧,
    })
}

fn スキン頂点属性を読む(
    入力: &mut 読取位置<'_>,
    頂点数: usize,
) -> Result<Option<Vec<スキン頂点属性>>, アセット実行時形式エラー> {
    match 入力.u8()? {
        0 => Ok(None),
        1 => {
            let 属性数 = 入力.件数(スキン頂点属性長)?;
            if 属性数 != 頂点数 {
                return Err(アセット実行時形式エラー::スキン頂点属性数不一致 { 属性数, 頂点数 });
            }
            let mut 一覧 = Vec::with_capacity(属性数);
            for _ in 0..属性数 {
                一覧.push(スキン頂点属性 {
                    ジョイント: [入力.u16()?, 入力.u16()?, 入力.u16()?, 入力.u16()?],
                    ウェイト: [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
                });
            }
            Ok(Some(一覧))
        }
        不正 => Err(アセット実行時形式エラー::不正な有無判別値(不正)),
    }
}

fn 頂点を読む(入力: &mut 読取位置<'_>) -> Result<メッシュ頂点属性, アセット実行時形式エラー> {
    Ok(メッシュ頂点属性 {
        位置: [入力.f32()?, 入力.f32()?, 入力.f32()?],
        法線: [入力.f32()?, 入力.f32()?, 入力.f32()?],
        接線: [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
        uv: [入力.f32()?, 入力.f32()?],
    })
}
