//! 性能計測系コマンドの割り当て。command_catalogの`benchmark`分類と同じ範囲(固定シーン・
//! 大規模世界・原点移動・地形段の一括計測など全体計測の10件)を担当する。

use std::process::ExitCode;

use crate::{bench, distant_view, large_world_bench, lod_crack, m10_bench, m11_soak, object_bench, origin_invariance, ow3_dod};

pub(super) fn 割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "bench" => Some(bench::実行する()),
        "bench-display-timing" => Some(bench::実表示計測つきで実行する()),
        "m10-bench" => Some(m10_bench::実行する()),
        "m11-soak" => Some(m11_soak::実行する()),
        "object-bench" => Some(object_bench::実行する()),
        "origin-invariance" => Some(origin_invariance::実行する()),
        "lod-crack" => Some(lod_crack::実行する()),
        "large-world-bench" => Some(large_world_bench::実行する(引数一覧)),
        "distant-view" => Some(distant_view::実行する(引数一覧)),
        "ow3-dod" => Some(ow3_dod::実行する()),
        _ => None,
    }
}
