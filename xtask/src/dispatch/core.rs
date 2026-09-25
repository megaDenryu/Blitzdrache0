//! 検証・規約系コマンドの割り当て。command_catalogの`core`分類と同じ範囲(検証列・規約検査・設計の命題の検証・
//! 型計測・起動スモーク確認・ビルドの中間データの掃除・対話メニューの7件)を担当する。

use std::process::ExitCode;

use crate::command_ui::menu;
use crate::{conform, design_verify, smoke, type_metrics, verify};

pub(super) fn 中核コマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "verify" => Some(verify::検証列を実行する()),
        "conform" => Some(conform::規約を検査する()),
        "design-verify" => Some(design_verify::設計の命題を検証する()),
        "type-metrics" => Some(type_metrics::型ごとの分量を計測する()),
        "smoke" => Some(smoke::スモークを実行する()),
        "clean-build-cache" => Some(verify::ビルドの中間データを掃除する(引数一覧)),
        "menu" => Some(menu::対話メニューを起動する()),
        _ => None,
    }
}
