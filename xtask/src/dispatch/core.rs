//! 検証・規約系コマンドの割り当て。command_catalogの`core`分類と同じ範囲(検証列・規約検査・
//! 型計測・起動スモーク確認・対話メニューの5件)を担当する。

use std::process::ExitCode;

use crate::command_ui::menu;
use crate::{conform, smoke, type_metrics, verify};

pub(super) fn 割り当てる(名前: &str, _引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "verify" => Some(verify::検証列を実行する()),
        "conform" => Some(conform::実行する()),
        "type-metrics" => Some(type_metrics::実行する()),
        "smoke" => Some(smoke::実行する()),
        "menu" => Some(menu::実行する()),
        _ => None,
    }
}
