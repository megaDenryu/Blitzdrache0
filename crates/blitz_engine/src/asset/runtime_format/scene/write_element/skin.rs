//! トポロジカル順のジョイント階層とバインド情報を書く。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use crate::asset::{joint::ジョイント, skin_data::スキンデータ, static_trs::静的TRS};

pub(in crate::asset::runtime_format::scene) fn 書く(
    出力: &mut 書込先,
    スキン: Option<&スキンデータ>,
) -> Result<(), アセット実行時形式エラー> {
    let Some(スキン) = スキン else {
        出力.u8(0);
        return Ok(());
    };
    if スキン.ジョイント一覧.is_empty() {
        return Err(アセット実行時形式エラー::ジョイントなし);
    }
    出力.u8(1);
    出力.件数(スキン.ジョイント一覧.len())?;
    for (添字, ジョイント) in スキン.ジョイント一覧.iter().enumerate() {
        ジョイントを書く(出力, ジョイント, 添字)?;
    }
    Ok(())
}

fn ジョイントを書く(
    出力: &mut 書込先, ジョイント: &ジョイント, ジョイント添字: usize
) -> Result<(), アセット実行時形式エラー> {
    match ジョイント.親添字 {
        None => 出力.u8(0),
        Some(親添字) if 親添字 < ジョイント添字 => {
            出力.u8(1);
            出力.件数(親添字)?;
        }
        Some(親添字) => {
            return Err(アセット実行時形式エラー::親添字順序違反 {
                ジョイント添字, 親添字
            });
        }
    }
    super::matrix::行列を書く(出力, ジョイント.逆バインド行列)?;
    trsを書く(出力, &ジョイント.バインド時)
}

fn trsを書く(出力: &mut 書込先, 値: &静的TRS) -> Result<(), アセット実行時形式エラー> {
    for 成分 in 値.平行移動.into_iter().chain(値.回転).chain(値.スケール) {
        出力.f32(成分)?;
    }
    Ok(())
}
