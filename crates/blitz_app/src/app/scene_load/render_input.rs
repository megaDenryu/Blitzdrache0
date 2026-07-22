//! engineの描画対象一覧をrenderの非空な描画シーン素材へ変換する。

mod layout;
#[cfg(test)]
mod layout_tests;

use blitz_engine::{シーンデータ, 描画対象データ};
use blitz_math::{ローカル, ワールド, 変換};
use blitz_render::{描画シーン素材, 描画対象素材};

use super::convert;
use crate::error::起動エラー;

pub(super) fn 変換する(
    シーン: &シーンデータ, 描画対象数: Option<crate::cli::描画対象数>
) -> Result<描画シーン素材, 起動エラー> {
    let 件数 = 描画対象数.map_or(シーン.描画対象一覧().len(), crate::cli::描画対象数::usize値);
    let mut 入力一覧 = (0..件数).map(|添字| {
        let 元 = &シーン.描画対象一覧()[添字 % シーン.描画対象一覧().len()];
        let 変換 = 描画対象数.map_or(元.ローカルからワールド(), |_| {
            元.ローカルからワールド().合成する(layout::配置する(添字, 件数))
        });
        描画対象を変換する(元, 変換)
    });
    let 先頭 = match 入力一覧.next() {
        Some(描画対象) => 描画対象?,
        None => panic!("シーンデータは1つ以上の描画対象を持つ不変条件に違反した"),
    };
    let 残り = 入力一覧.collect::<Result<Vec<_>, _>>()?;
    Ok(描画シーン素材::生成する(先頭, 残り))
}

fn 描画対象を変換する(
    描画対象: &描画対象データ,
    ローカルからワールド: 変換<ローカル, ワールド>,
) -> Result<描画対象素材, 起動エラー> {
    let メッシュ = 描画対象.メッシュ();
    let 頂点一覧 = メッシュ.頂点一覧.iter().map(convert::頂点変換する).collect();
    let マテリアル = convert::マテリアルを変換する(描画対象.マテリアル())?;
    Ok(描画対象素材::生成する(
        ローカルからワールド,
        頂点一覧,
        メッシュ.インデックス一覧.clone(),
        マテリアル,
    ))
}
