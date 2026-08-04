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
mod count_consistency;
mod csm_seam;
mod damage_band;
mod day_moment;
mod derived_environment;
mod dispatch;
mod distant_environment;
mod fetch_assets;
mod file_scan;
mod gen_atmosphere_reference;
mod gen_source_assets;
mod indirect_cost;
mod indirect_probe;
mod instance_cull;
mod instance_draw;
mod instance_lod;
mod instance_stream;
mod lod_crack;
mod m10_bench;
mod m11_soak;
mod material_reload_draw;
mod memory_sampling;
mod multi_material_draw;
mod object_bench;
mod origin_invariance;
mod ow3_dod;
mod ow4_bench;
mod pixel_region;
mod plate_region;
mod prop_draw;
mod prop_multi_material_draw;
mod raw_image;
mod raw_png;
mod release_build;
mod report_parse;
mod shader_copy;
mod shader_reload_draw;
mod shadow_loss;
mod shadow_probe;
mod sky_draw;
mod sky_lut;
mod sky_state;
mod sky_time;
mod smoke;
mod streaming_bench;
mod streaming_report;
mod terrain_visual;
mod type_metrics;
mod usage;
mod validation_count;
mod vegetation_run;
mod verify;
mod vertex_diag;
mod village_draw;
mod watch_assets;

fn main() -> ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    dispatch::割り当てる(&引数一覧)
}
