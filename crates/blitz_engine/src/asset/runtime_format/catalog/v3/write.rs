//! 版3のカタログ内容を検査用に組み立てる。

use super::super::super::scene::bytes::書込先;
use super::super::super::アセット実行時形式エラー;
use super::super::bytes;
use crate::asset::カタログ;

pub(in crate::asset::runtime_format::catalog) fn カタログ内容を書く(
    カタログ: &カタログ,
) -> Result<Vec<u8>, アセット実行時形式エラー> {
    let 一覧 = bytes::安定id順に並べる(カタログ);
    let mut 出力 = 書込先::新規();
    出力.件数(一覧.len())?;
    for (id, 項目) in 一覧 {
        bytes::項目本体を書く(&mut 出力, id, 項目)?;
        let メタデータ = 項目.メタデータ();
        出力.u64(メタデータ.頂点数);
        出力.u64(メタデータ.インデックス数);
        出力.u64(メタデータ.テクスチャ格納バイト数);
        出力.u64(メタデータ.個体数);
    }
    Ok(出力.完了する())
}
