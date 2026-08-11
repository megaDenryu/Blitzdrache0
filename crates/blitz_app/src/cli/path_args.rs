//! 役割を持つパスを値とするCLI引数の解析。担当するのは`--shader-source`と`--asset-root`の2つであり、
//! どちらも読んだ綴りをそのまま役割の型へ包んで返す。
//!
//! 数や列挙を読む`value_args`と分けるのは、この2つだけが「同じ`PathBuf`の姿をした別物」を作る入口であり、
//! 包む先の型を取り違えても数の解析のように失敗しないためである。

use std::slice::Iter;

use super::{value_args, 起動引数エラー};
use crate::hot_reload::監視するシェーダーの入口ファイル;
use crate::runtime_assets::実行時アセットの置き場;

pub(super) fn shader_source引数を処理する(
    引数: &mut Iter<String>,
) -> Result<監視するシェーダーの入口ファイル, 起動引数エラー> {
    let 値 = value_args::次の値を読む(引数, "--shader-source", 起動引数エラー::シェーダーソース不正)?;
    Ok(監視するシェーダーの入口ファイル::綴りから生成する(値))
}

pub(super) fn asset_root引数を処理する(引数: &mut Iter<String>) -> Result<実行時アセットの置き場, 起動引数エラー> {
    let 値 = value_args::次の値を読む(引数, "--asset-root", 起動引数エラー::アセットルート不正)?;
    Ok(実行時アセットの置き場::綴りから生成する(値))
}
