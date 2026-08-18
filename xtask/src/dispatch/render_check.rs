//! 描画検収系コマンドの割り当て。command_catalogの`render_check`分類と同じ範囲(小物・植生・布・
//! 光源・材質差し替えなど実機描画した絵と計器値で正しさを確かめる23件)を担当する。

use std::process::ExitCode;

use crate::{
    auto_exposure, cloth_empty, cloth_night, cloth_shadow_order, cluster_lights, csm_seam, hdr_luminance, ibl_step, instance_cull, instance_draw,
    instance_lod, instance_stream, material_reload_draw, multi_material_draw, point_light_shadow, prop_draw, prop_multi_material_draw,
    shader_reload_draw, temporal_visual, terrain_visual, texture_compression, vertex_diag, village_draw,
};

pub(super) fn 割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "prop-draw" => Some(prop_draw::実行する()),
        "prop-multi-material-draw" => Some(prop_multi_material_draw::実行する()),
        "village-draw" => Some(village_draw::実行する()),
        "terrain-visual" => Some(terrain_visual::実行する()),
        "temporal-visual" => Some(temporal_visual::実行する()),
        "texture-compression" => Some(texture_compression::実行する()),
        "hdr-luminance" => Some(hdr_luminance::実行する()),
        "auto-exposure" => Some(auto_exposure::実行する()),
        "ibl-step" => Some(ibl_step::実行する()),
        "instance-draw" => Some(instance_draw::実行する()),
        "instance-cull" => Some(instance_cull::実行する()),
        "instance-lod" => Some(instance_lod::実行する()),
        "multi-material-draw" => Some(multi_material_draw::実行する()),
        "material-reload-draw" => Some(material_reload_draw::実行する()),
        "shader-reload-draw" => Some(shader_reload_draw::実行する()),
        "instance-stream" => Some(instance_stream::実行する()),
        "cloth-empty" => Some(cloth_empty::実行する()),
        "cloth-night" => Some(cloth_night::実行する()),
        "cluster-lights" => Some(cluster_lights::実行する()),
        "point-light-shadow" => Some(point_light_shadow::実行する()),
        "cloth-shadow-order" => Some(cloth_shadow_order::実行する()),
        "csm-seam" => Some(csm_seam::実行する(引数一覧)),
        "vertex-diag" => Some(vertex_diag::実行する()),
        _ => None,
    }
}
