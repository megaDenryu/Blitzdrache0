//! 描画検収系コマンドの割り当て。command_catalogの`render_check`分類と同じ範囲(小物・植生・布・光源など
//! 実機描画した絵と計器値で正しさを確かめる21件)を担当する。材質の絵の検収は`material_check`が持つ。

use std::process::ExitCode;

use crate::{
    auto_exposure, cloth_empty, cloth_night, cloth_shadow_order, cloth_xpbd_reference, cluster_lights, csm_seam, hdr_luminance, ibl_step,
    instance_cull, instance_draw, instance_lod, instance_stream, point_light_shadow, prop_draw, shader_reload_draw, temporal_visual, terrain_visual,
    texture_compression, vertex_diag, village_draw,
};

pub(super) fn 描画検収コマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "prop-draw" => Some(prop_draw::小物1体の描画を確認する()),
        "village-draw" => Some(village_draw::見本集落の描画を確認する()),
        "terrain-visual" => Some(terrain_visual::地形庭を目視確認する()),
        "temporal-visual" => Some(temporal_visual::時間再構成を目視確認する()),
        "texture-compression" => Some(texture_compression::テクスチャ圧縮を確認する()),
        "hdr-luminance" => Some(hdr_luminance::hdr輝度を実測する()),
        "auto-exposure" => Some(auto_exposure::自動露出を判定する()),
        "ibl-step" => Some(ibl_step::遠方環境更新境界を実測する()),
        "instance-draw" => Some(instance_draw::植生インスタンスの描画を確認する()),
        "instance-cull" => Some(instance_cull::植生の可視判定を確認する()),
        "instance-lod" => Some(instance_lod::植生段の切替を確認する()),
        "shader-reload-draw" => Some(shader_reload_draw::シェーダー差し替えの描画を確認する()),
        "instance-stream" => Some(instance_stream::地形植生ストリーミングを確認する()),
        "cloth-empty" => Some(cloth_empty::視錐台外の布描画を確認する()),
        "cloth-night" => Some(cloth_night::布ライティングの追従を確認する()),
        "cluster-lights" => Some(cluster_lights::多光源クラスタを確認する()),
        "point-light-shadow" => Some(point_light_shadow::点光源の影を確認する()),
        "cloth-shadow-order" => Some(cloth_shadow_order::布の影の走査順を確認する()),
        "cloth-xpbd-reference" => Some(cloth_xpbd_reference::布のxpbd参照比較を確認する()),
        "csm-seam" => Some(csm_seam::距離区分影の境界を確認する(引数一覧)),
        "vertex-diag" => Some(vertex_diag::頂点量の診断を確認する()),
        _ => None,
    }
}
