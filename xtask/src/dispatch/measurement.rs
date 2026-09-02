//! 律速切り分け計測系コマンドの割り当て。command_catalogの`measurement`分類と同じ範囲
//! (影・間接照明・深度プリパス・ストリーミング・XPBDの並列方式など条件別の切り分け計測8件)を担当する。

use std::process::ExitCode;

use crate::{depth_prepass_cost, indirect_cost, ow4_bench, reverse_depth, shadow_loss, shadow_probe, streaming_bench, xpbd_solver_bench};

pub(super) fn 律速切り分け計測コマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "ow4-bench" => Some(ow4_bench::植生密度別に性能を計測する(引数一覧)),
        "shadow-probe" => Some(shadow_probe::影の律速切り分けを計測する(引数一覧)),
        "indirect-cost" => Some(indirect_cost::間接照明費用を計測する(引数一覧)),
        "depth-prepass-cost" => Some(depth_prepass_cost::深度プリパス費用を計測する(引数一覧)),
        "reverse-depth" => Some(reverse_depth::反転深度を撮影して判定する(引数一覧)),
        "shadow-loss" => Some(shadow_loss::影の欠落を計測する(引数一覧)),
        "streaming-bench" => Some(streaming_bench::ストリーミング経路の資源を計測する(引数一覧)),
        "xpbd-solver-bench" => Some(xpbd_solver_bench::xpbd並列方式を計測する(引数一覧)),
        _ => None,
    }
}
