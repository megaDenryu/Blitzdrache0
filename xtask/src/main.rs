//! 開発ツールの唯一の入口。`cargo xtask <コマンド>` で呼ぶ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」

use std::process::ExitCode;

mod atmosphere_lut;
mod bench;
mod check_glb;
mod cloth_empty;
mod cloth_night;
mod cloth_shadow_order;
mod compile_assets;
mod conform;
mod csm_seam;
mod fetch_assets;
mod file_scan;
mod gen_atmosphere_reference;
mod gen_source_assets;
mod instance_cull;
mod instance_draw;
mod instance_lod;
mod instance_stream;
mod lod_crack;
mod m10_bench;
mod m11_soak;
mod memory_sampling;
mod object_bench;
mod origin_invariance;
mod ow3_dod;
mod ow4_bench;
mod pixel_region;
mod prop_draw;
mod raw_image;
mod raw_png;
mod release_build;
mod report_parse;
mod shader_copy;
mod sky_draw;
mod sky_lut;
mod sky_state;
mod sky_time;
mod smoke;
mod streaming_bench;
mod streaming_report;
mod type_metrics;
mod usage;
mod validation_count;
mod vegetation_run;
mod verify;
mod watch_assets;

fn main() -> ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match 引数一覧.first().map(String::as_str) {
        Some("verify") => verify::検証列を実行する(),
        Some("conform") => conform::実行する(),
        Some("type-metrics") => type_metrics::実行する(),
        Some("smoke") => smoke::実行する(),
        Some("check-glb") => check_glb::実行する(&引数一覧[1..]),
        Some("compile-assets") => compile_assets::実行する(&引数一覧[1..]),
        Some("watch-assets") => watch_assets::実行する(&引数一覧[1..]),
        Some("gen-source-assets") => gen_source_assets::実行する(),
        Some("gen-atmosphere-reference") => gen_atmosphere_reference::実行する(&引数一覧[1..]),
        Some("fetch-assets") => fetch_assets::実行する(),
        Some("bench") => bench::実行する(),
        Some("bench-display-timing") => bench::実表示計測つきで実行する(),
        Some("m10-bench") => m10_bench::実行する(),
        Some("m11-soak") => m11_soak::実行する(),
        Some("object-bench") => object_bench::実行する(),
        Some("origin-invariance") => origin_invariance::実行する(),
        Some("lod-crack") => lod_crack::実行する(),
        Some("prop-draw") => prop_draw::実行する(),
        Some("instance-draw") => instance_draw::実行する(),
        Some("instance-cull") => instance_cull::実行する(),
        Some("instance-lod") => instance_lod::実行する(),
        Some("instance-stream") => instance_stream::実行する(),
        Some("cloth-empty") => cloth_empty::実行する(),
        Some("cloth-night") => cloth_night::実行する(),
        Some("cloth-shadow-order") => cloth_shadow_order::実行する(),
        Some("csm-seam") => csm_seam::実行する(),
        Some("sky-draw") => sky_draw::実行する(),
        Some("sky-state") => sky_state::実行する(),
        Some("sky-lut") => sky_lut::実行する(&引数一覧[1..]),
        Some("atmosphere-lut") => atmosphere_lut::実行する(),
        Some("sky-time") => sky_time::実行する(),
        Some("ow3-dod") => ow3_dod::実行する(),
        Some("ow4-bench") => ow4_bench::実行する(&引数一覧[1..]),
        Some("streaming-bench") => streaming_bench::実行する(&引数一覧[1..]),
        _ => {
            usage::使い方を表示する();
            ExitCode::FAILURE
        }
    }
}
