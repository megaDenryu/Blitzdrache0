//! `egui::ClippedPrimitive`列をblitz_render::UI描画データへ変換する。
//! `Primitive::Callback`(カスタム描画コールバック)は開発用UIでは使用しないため対象外。

use super::scissor_convert;
use super::texture_id_map;

pub(super) fn 変換する(一覧: &[egui::ClippedPrimitive], pixels_per_point: f32) -> blitz_render::UI描画データ {
    let メッシュ一覧 = 一覧.iter().filter_map(|項目| メッシュへ変換する(項目, pixels_per_point)).collect();
    blitz_render::UI描画データ { メッシュ一覧 }
}

fn メッシュへ変換する(項目: &egui::ClippedPrimitive, pixels_per_point: f32) -> Option<blitz_render::UIメッシュ> {
    let egui::epaint::Primitive::Mesh(mesh) = &項目.primitive else {
        return None;
    };
    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
        return None;
    }
    let 頂点一覧 = mesh
        .vertices
        .iter()
        .map(|頂点| blitz_render::UI頂点 {
            位置px: [頂点.pos.x * pixels_per_point, 頂点.pos.y * pixels_per_point],
            uv: [頂点.uv.x, 頂点.uv.y],
            色rgba8: 頂点.color.to_array(),
        })
        .collect();
    Some(blitz_render::UIメッシュ {
        頂点一覧,
        インデックス一覧: mesh.indices.clone(),
        テクスチャid: texture_id_map::変換する(mesh.texture_id),
        シザー矩形px: scissor_convert::変換する(項目.clip_rect, pixels_per_point),
    })
}
