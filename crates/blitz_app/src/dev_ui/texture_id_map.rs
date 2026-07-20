//! egui::TextureIdからblitz_render::UIテクスチャIDへの決定的な写像。
//! ManagedとUserの両系列が同じu64空間で衝突しないよう、偶奇で系列を分ける。

pub(super) fn 変換する(id: egui::TextureId) -> blitz_render::UIテクスチャID {
    let 値 = match id {
        egui::TextureId::Managed(番号) => 番号.saturating_mul(2),
        egui::TextureId::User(番号) => 番号.saturating_mul(2).saturating_add(1),
    };
    blitz_render::UIテクスチャID::生成する(値)
}
