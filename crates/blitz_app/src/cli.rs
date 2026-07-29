//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。
//! 型定義は`types`、値を伴う引数の個別処理は`value_args`に委ねる。

#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod launch_request_tests;

mod argument_error;
mod instance_lod_args;
mod launch_request;
mod lod_crack_args;
mod modes;
mod object_count;
mod placement_args;
#[cfg(test)]
mod placement_args_tests;
mod setting_apply;
mod streaming_settings;
mod time_of_day_settings;
mod types;
mod value_args;
pub(crate) use argument_error::起動引数エラー;
pub(crate) use launch_request::起動要求;
pub(crate) use modes::{布モード, 粒子表示モード};
pub(crate) use object_count::描画対象数;
pub(crate) use placement_args::平行移動起動設定;
pub(crate) use streaming_settings::{LOD継ぎ目検査設定, ストリーミング起動設定, プレイヤー位置源};
pub(crate) use time_of_day_settings::{時間帯起動設定, 空の起動指定};
pub(crate) use types::{起動モード, 起動設定};

use crate::error::起動エラー;

/// 描画しない報告だけを求める引数。描画の設定を1つも読まないため、他の引数より先に判定する。
const 天空状態報告引数: &str = "--report-sky-state";

/// CLI引数から起動要求を解析する。粒子系の検証対象は`--particles`または`--surface-flow`で選ぶ。
/// `--shader-source`は監視・再コンパイル対象のエントリファイルを指す。`import`先の他ファイルは常にエントリと同じディレクトリから解決するため個別指定は不要。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動要求, 起動エラー> {
    if 引数一覧.iter().any(|引数値| 引数値 == 天空状態報告引数) {
        return Ok(起動要求::天空状態報告);
    }
    Ok(起動要求::描画実行(Box::new(起動設定を解析する(引数一覧)?)))
}

fn 起動設定を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut 起動設定 = 起動設定::既定値();

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        setting_apply::反映する(&mut 起動設定, 引数値, &mut 引数)?;
    }

    Ok(起動設定)
}
