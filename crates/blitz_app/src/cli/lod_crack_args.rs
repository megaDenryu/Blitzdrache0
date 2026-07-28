//! GPU継ぎ目検査が指定する隣接2チャンクと詳細段を、CLI文字列から型付き設定へ変換する。

use std::slice::Iter;

use blitz_engine::チャンク座標;
use blitz_render::地形詳細段;

use super::value_args::次の値を読む;
use super::{LOD継ぎ目検査設定, 起動引数エラー};

pub(super) fn 引数を処理する(引数: &mut Iter<String>) -> Result<LOD継ぎ目検査設定, 起動引数エラー> {
    let x1 = i32を読む(引数, "一方X")?;
    let z1 = i32を読む(引数, "一方Z")?;
    let 段1 = u8を読む(引数, "一方段")?;
    let x2 = i32を読む(引数, "他方X")?;
    let z2 = i32を読む(引数, "他方Z")?;
    let 段2 = u8を読む(引数, "他方段")?;
    Ok(LOD継ぎ目検査設定 {
        一方座標: チャンク座標::生成する(x1, z1),
        一方段: 地形詳細段::番号から生成する(段1),
        他方座標: チャンク座標::生成する(x2, z2),
        他方段: 地形詳細段::番号から生成する(段2),
    })
}

fn i32を読む(引数: &mut Iter<String>, 名前: &str) -> Result<i32, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--lod-crack-pair", 起動引数エラー::Lod継ぎ目検査不正)?;
    値.parse().map_err(|_| 起動引数エラー::Lod継ぎ目検査不正(format!("{名前}: {値}")))
}

fn u8を読む(引数: &mut Iter<String>, 名前: &str) -> Result<u8, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--lod-crack-pair", 起動引数エラー::Lod継ぎ目検査不正)?;
    let 段 = 値.parse().map_err(|_| 起動引数エラー::Lod継ぎ目検査不正(format!("{名前}: {値}")))?;
    if 段 > 4 {
        return Err(起動引数エラー::Lod継ぎ目検査不正(format!("{名前}が最粗段4を超える: {段}")));
    }
    Ok(段)
}
