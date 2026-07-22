//! 描画対象の識別子、変換、メッシュ、材質を書く。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use crate::asset::render_object_data::描画対象データ;

pub(super) fn 書く(
    出力: &mut 書込先, 対象: &描画対象データ, ジョイント数: Option<usize>
) -> Result<(), アセット実行時形式エラー> {
    出力.u64(対象.識別子().番号を返す());
    出力.u64(対象.所有チャンク().番号を返す());
    for 列 in 対象.ローカルからワールド().gpu境界用列優先配列() {
        for 値 in 列 {
            出力.f32(値)?;
        }
    }
    super::mesh::書く(出力, 対象.メッシュ(), ジョイント数)?;
    super::material::書く(出力, 対象.マテリアル())
}
