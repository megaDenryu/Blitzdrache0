//! 律速切り分け計測系コマンドの割り当て。command_catalogの`measurement`分類と同じ範囲
//! (影・間接照明・深度プリパス・ストリーミングなど条件別の切り分け計測7件)を担当する。

use std::process::ExitCode;

use crate::{depth_prepass_cost, indirect_cost, ow4_bench, reverse_depth, shadow_loss, shadow_probe, streaming_bench};

pub(super) fn 割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "ow4-bench" => Some(ow4_bench::実行する(引数一覧)),
        "shadow-probe" => Some(shadow_probe::実行する(引数一覧)),
        "indirect-cost" => Some(indirect_cost::実行する(引数一覧)),
        "depth-prepass-cost" => Some(depth_prepass_cost::実行する(引数一覧)),
        "reverse-depth" => Some(reverse_depth::実行する(引数一覧)),
        "shadow-loss" => Some(shadow_loss::実行する(引数一覧)),
        "streaming-bench" => Some(streaming_bench::実行する(引数一覧)),
        _ => None,
    }
}
