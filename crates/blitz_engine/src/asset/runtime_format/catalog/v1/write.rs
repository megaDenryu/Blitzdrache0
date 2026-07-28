//! 版1のカタログバイト列を組み立てる。旧版を読んで最新版へ変換する経路を検査するには版1の実物が要るため、その材料をここが作る。
//! 実行時形式の書き出しは常に最新版で行うため、この経路は検査からのみ呼ぶ。

use super::super::super::scene::bytes::書込先;
use super::super::super::アセット実行時形式エラー;
use super::super::bytes;
use crate::asset::catalog::カタログ;

pub(in crate::asset::runtime_format::catalog) fn 内容を書く(
    カタログ: &カタログ
) -> Result<Vec<u8>, アセット実行時形式エラー> {
    let 一覧 = bytes::安定id順に並べる(カタログ);
    let mut 出力 = 書込先::新規();
    出力.件数(一覧.len())?;
    for (id, 項目) in 一覧 {
        bytes::項目本体を書く(&mut 出力, id, 項目)?;
        let メタデータ = 項目.メタデータ();
        出力.u64(メタデータ.頂点数);
        出力.u64(メタデータ.インデックス数);
        出力.u64(メタデータ.テクスチャバイト数);
    }
    Ok(出力.完了する())
}
