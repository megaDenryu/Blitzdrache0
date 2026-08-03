//! コマンド名から実装への割り当て。担当するのは対応表だけであり、モジュール構成は`main`が、説明文は`usage`が持つ。
//! 表を`main`から分けるのは、コマンドが増えるたびにモジュール宣言と対応表が同じファイルで押し合うためである。

use std::process::ExitCode;

use crate::{
    atmosphere_lut, bench, check_glb, cloth_empty, cloth_night, cloth_shadow_order, compile_assets, conform, csm_seam, derived_environment,
    distant_environment, fetch_assets, gen_atmosphere_reference, gen_source_assets, indirect_probe, instance_cull, instance_draw, instance_lod,
    instance_stream, lod_crack, m10_bench, m11_soak, material_reload_draw, multi_material_draw, object_bench, origin_invariance, ow3_dod, ow4_bench,
    prop_draw, prop_multi_material_draw, shader_reload_draw, shadow_loss, shadow_probe, sky_draw, sky_lut, sky_state, sky_time, smoke,
    streaming_bench, terrain_visual, type_metrics, usage, verify, vertex_diag, village_draw, watch_assets,
};

pub(crate) fn 割り当てる(引数一覧: &[String]) -> ExitCode {
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
        Some("prop-multi-material-draw") => prop_multi_material_draw::実行する(),
        Some("village-draw") => village_draw::実行する(),
        Some("terrain-visual") => terrain_visual::実行する(),
        Some("instance-draw") => instance_draw::実行する(),
        Some("instance-cull") => instance_cull::実行する(),
        Some("instance-lod") => instance_lod::実行する(),
        Some("multi-material-draw") => multi_material_draw::実行する(),
        Some("material-reload-draw") => material_reload_draw::実行する(),
        Some("shader-reload-draw") => shader_reload_draw::実行する(),
        Some("instance-stream") => instance_stream::実行する(),
        Some("cloth-empty") => cloth_empty::実行する(),
        Some("cloth-night") => cloth_night::実行する(),
        Some("cloth-shadow-order") => cloth_shadow_order::実行する(),
        Some("csm-seam") => csm_seam::実行する(),
        Some("vertex-diag") => vertex_diag::実行する(),
        Some("sky-draw") => sky_draw::実行する(),
        Some("sky-state") => sky_state::実行する(),
        Some("sky-lut") => sky_lut::実行する(&引数一覧[1..]),
        Some("atmosphere-lut") => atmosphere_lut::実行する(),
        Some("distant-environment") => distant_environment::実行する(),
        Some("derived-environment") => derived_environment::実行する(),
        Some("indirect-probe") => indirect_probe::実行する(),
        Some("sky-time") => sky_time::実行する(),
        Some("ow3-dod") => ow3_dod::実行する(),
        Some("ow4-bench") => ow4_bench::実行する(&引数一覧[1..]),
        Some("shadow-probe") => shadow_probe::実行する(&引数一覧[1..]),
        Some("shadow-loss") => shadow_loss::実行する(&引数一覧[1..]),
        Some("streaming-bench") => streaming_bench::実行する(&引数一覧[1..]),
        _ => {
            usage::使い方を表示する();
            ExitCode::FAILURE
        }
    }
}
