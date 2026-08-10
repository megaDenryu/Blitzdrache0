//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。
//! 型定義は`types`、値を伴う引数の個別処理は`value_args`、引数一覧から起動要求を組み立てる工程は`parse`に委ねる。

#[cfg(test)]
mod cli_tests;
#[cfg(test)]
mod frame_dump_args_tests;
#[cfg(test)]
mod game_selection_tests;
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
mod game_operation_source;
mod game_selection;
mod ibl_step_scan_args;
mod indirect_probe_args;
mod instance_lod_args;
mod launch_request;
mod lighting_settings;
mod local_light_count_args;
pub(crate) mod local_visibility_settings;
mod lod_crack_args;
mod modes;
mod object_count;
mod parse;
mod placement_args;
#[cfg(test)]
mod placement_args_tests;
mod point_light_shadow_count_args;
mod report_only_request;
mod screen_pixel_args;
mod setting_apply;
mod shadow_args;
mod streaming_settings;
pub(crate) mod temporal_reconstruction_settings;
mod time_args;
mod time_of_day_flags;
mod time_of_day_settings;
mod types;
mod value_args;
mod verification_plan;
pub(crate) use argument_error::起動引数エラー;
pub(crate) use draw_object_layout::描画対象の並べ方;
pub(crate) use draw_object_order::描画対象の走査順;
pub(crate) use game_operation_source::ゲーム操作の出どころ;
pub(crate) use game_selection::遊ぶゲームの指定;
pub(crate) use launch_request::起動要求;
pub(crate) use modes::{布モード, 粒子表示モード};
pub(crate) use object_count::描画対象数;
pub(crate) use parse::引数を解析する;
pub(crate) use placement_args::平行移動起動設定;
pub(crate) use screen_pixel_args::画面画素位置;
pub(crate) use shadow_args::シャドウ計測起動設定;
pub(crate) use streaming_settings::{LOD継ぎ目検査設定, ストリーミング起動設定, プレイヤー位置源};
pub(crate) use time_of_day_settings::{
    太陽円盤指定, 時間帯起動設定, 空の起動指定, 空中遠近合成指定, 自動露出の起動指定
};
pub(crate) use types::{フレームダンプ指定, 読み戻し検収起動設定, 起動モード, 起動設定};
pub(crate) use verification_plan::検証計画指定;
pub(crate) use {
    local_light_count_args::局所光の件数の起動指定, point_light_shadow_count_args::影を落とす灯の件数の起動指定
};
