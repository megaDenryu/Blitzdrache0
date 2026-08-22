//! 性能計測系コマンドの割り当て。command_catalogの`benchmark`分類と同じ範囲(固定シーン・
//! 大規模世界・原点移動・地形段の一括計測など全体計測の10件)を担当する。

use std::process::ExitCode;

use crate::{bench, distant_view, large_world_bench, lod_crack, m10_bench, m11_soak, object_bench, origin_invariance, ow3_dod};

pub(super) fn 性能計測コマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "bench" => Some(bench::固定シーンを計測する()),
        "bench-display-timing" => Some(bench::実表示計測つきで実行する()),
        "m10-bench" => Some(m10_bench::流体試作を計測する()),
        "m11-soak" => Some(m11_soak::連続実行のメモリ推移を計測する()),
        "object-bench" => Some(object_bench::対象数別に性能を計測する()),
        "origin-invariance" => Some(origin_invariance::原点移動の不変性を確認する()),
        "lod-crack" => Some(lod_crack::地形段差の継ぎ目を確認する()),
        "large-world-bench" => Some(large_world_bench::大規模世界の固定経路を計測する(引数一覧)),
        "distant-view" => Some(distant_view::遠景を撮影して判定する(引数一覧)),
        "ow3-dod" => Some(ow3_dod::原点移動と地形段を一括計測する()),
        _ => None,
    }
}
