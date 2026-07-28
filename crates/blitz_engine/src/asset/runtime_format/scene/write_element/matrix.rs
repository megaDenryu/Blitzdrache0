//! 4行4列の列優先行列を書く。描画対象の配置とジョイントの逆バインド行列が同じ並びを使う。

use blitz_math::変換;

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;

pub(in crate::asset::runtime_format::scene) fn 行列を書く<元, 先>(
    出力: &mut 書込先,
    変換: 変換<元, 先>,
) -> Result<(), アセット実行時形式エラー> {
    for 列 in 変換.gpu境界用列優先配列() {
        for 値 in 列 {
            出力.f32(値)?;
        }
    }
    Ok(())
}
