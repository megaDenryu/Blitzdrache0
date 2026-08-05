//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。型定義は`types`、値を伴う引数の個別処理は`value_args`に委ねる。

#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod frame_dump_args_tests;
#[cfg(test)]
mod ibl_step_scan_args_tests;
#[cfg(test)]
mod launch_request_tests;
#[cfg(test)]
mod local_visibility_args_tests;
#[cfg(test)]
mod report_only_request_tests;
#[cfg(test)]
mod time_args_tests;

mod argument_error;
mod auto_exposure_probe_args;
mod depth_prepass_args;
mod draw_object_layout;
mod draw_object_order;
mod ibl_step_scan_args;
mod indirect_probe_args;
mod instance_lod_args;
mod launch_request;
pub(crate) mod local_visibility_settings;
mod lod_crack_args;
mod modes;
mod object_count;
mod placement_args;
#[cfg(test)]
mod placement_args_tests;
mod report_only_request;
mod screen_pixel_args;
mod setting_apply;
mod shadow_args;
mod streaming_settings;
mod time_args;
mod time_of_day_flags;
mod time_of_day_settings;
mod types;
mod value_args;
mod verification_plan;
pub(crate) use argument_error::起動引数エラー;
pub(crate) use draw_object_layout::描画対象の並べ方;
pub(crate) use draw_object_order::描画対象の走査順;
pub(crate) use launch_request::起動要求;
pub(crate) use modes::{布モード, 粒子表示モード};
pub(crate) use object_count::描画対象数;
pub(crate) use placement_args::平行移動起動設定;
pub(crate) use screen_pixel_args::画面画素位置;
pub(crate) use shadow_args::シャドウ計測起動設定;
pub(crate) use streaming_settings::{LOD継ぎ目検査設定, ストリーミング起動設定, プレイヤー位置源};
pub(crate) use time_of_day_settings::{
    太陽円盤指定, 時間帯起動設定, 空の起動指定, 空中遠近合成指定, 自動露出の起動指定
};
pub(crate) use types::{フレームダンプ指定, 読み戻し検収起動設定, 起動モード, 起動設定};
pub(crate) use verification_plan::検証計画指定;

use crate::error::起動エラー;

/// CLI引数から起動要求を解析する。粒子系の検証対象は`--particles`または`--surface-flow`で選ぶ。
/// `--shader-source`は監視・再コンパイル対象のエントリファイルを指す。`import`先の他ファイルは常にエントリと同じディレクトリから解決するため個別指定は不要。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動要求, 起動エラー> {
    if let Some(要求) = report_only_request::報告だけの要求を見分ける(引数一覧) {
        return Ok(要求);
    }
    Ok(起動要求::描画実行(Box::new(起動設定を解析する(引数一覧)?)))
}

/// 引数を全部読み終えてから、組み合わせの成立を確かめる。走査の途中で確かめられないのは、どの検査も
/// 「後から来る引数が無いこと」を条件に含むためである。指定の順序で結果が変わる検査は、利用者が並べ方を
/// 覚えていなければならない仕様になる。
fn 起動設定を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut 起動設定 = 起動設定::既定値();

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        setting_apply::反映する(&mut 起動設定, 引数値, &mut 引数)?;
    }
    types::走査の書き出し先を確かめる(起動設定.モード, &起動設定.フレームダンプ先)?;
    local_visibility_settings::検収とフレームダンプの排他を確かめる(
        起動設定.読み戻し検収.局所可視性の検収の形,
        &起動設定.フレームダンプ先,
    )?;

    Ok(起動設定)
}
