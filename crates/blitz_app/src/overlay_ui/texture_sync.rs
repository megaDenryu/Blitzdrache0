//! egui::TexturesDeltaをレンダラーのUIテクスチャ登録/削除へ反映する。

use super::texture_id_map;
use super::texture_mirror::テクスチャミラー;
use crate::error::起動エラー;

pub(super) fn 反映する(
    レンダラー: &mut blitz_render::レンダラー,
    デルタ: &egui::TexturesDelta,
    ミラー: &mut テクスチャミラー,
) -> Result<(), 起動エラー> {
    for (id, パッチ) in &デルタ.set {
        let (幅, 高さ, rgba8) = ミラー.反映して全体を得る(*id, パッチ);
        let 幅u32 = usize要素をu32へ変換する(幅);
        let 高さu32 = usize要素をu32へ変換する(高さ);
        let 素材 = blitz_render::UIテクスチャ素材::生成する(幅u32, 高さu32, rgba8)?;
        レンダラー.uiテクスチャを登録する(texture_id_map::変換する(*id), 素材)?;
    }
    for id in &デルタ.free {
        ミラー.削除する(*id);
        レンダラー.uiテクスチャを削除する(texture_id_map::変換する(*id));
    }
    Ok(())
}

/// テクスチャ寸法(egui由来、実運用でu32に収まる)をu32へ変換する。
fn usize要素をu32へ変換する(値: usize) -> u32 {
    u32::try_from(値).unwrap_or_else(|_| panic!("UIテクスチャ寸法がu32に収まらない: {値}"))
}
