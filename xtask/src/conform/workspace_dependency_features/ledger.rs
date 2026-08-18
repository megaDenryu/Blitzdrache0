//! 依存ごとに許される機能の台帳。担当するのは、どの依存がどの機能を有効にしてよいかの正本を持つことだけである。
//!
//! 検査の工程から分けているのは、依存や機能を足すときに触るのがこの台帳だけであり、工程の側は触らないためである。
//! 触れる対象と変わる理由が別であることが分ける根拠である。

/// 台帳の1行。const の表として持つ記録であり、振る舞いを持たないためフィールドをそのまま読ませる。
pub(super) struct 機能台帳の項 {
    pub(super) 依存名: &'static str,
    pub(super) 既定機能を使うか: bool,
    pub(super) 機能一覧: &'static [&'static str],
}

/// 既定機能を切るのはimageとegui-winitの2件、機能を明示するのはimageとgltfとserde_jsonとserdeとtokioとtower-httpの6件である。残りは版だけを綴る。
pub(super) const 機能台帳: [機能台帳の項; 19] = [
    項を作る("ash", true, &[]),
    項を作る("ash-window", true, &[]),
    項を作る("winit", true, &[]),
    項を作る("raw-window-handle", true, &[]),
    項を作る("glam", true, &[]),
    項を作る("thiserror", true, &[]),
    項を作る("gltf", true, &["extras"]),
    項を作る("image", false, &["png", "jpeg"]),
    項を作る("serde_json", true, &["raw_value"]),
    項を作る("rayon", true, &[]),
    項を作る("egui", true, &[]),
    項を作る("egui-winit", false, &[]),
    項を作る("crossterm", true, &[]),
    // ゲーム開発用エディター段1で新設。editor_serverのみが依存する
    項を作る("serde", true, &["derive"]),
    項を作る("axum", true, &[]),
    項を作る("tokio", true, &["rt-multi-thread", "macros", "net", "fs"]),
    項を作る("tower", true, &[]),
    項を作る("tower-http", true, &["fs"]),
    項を作る("ts-rs", true, &[]),
];

pub(super) const fn 項を作る(
    依存名: &'static str, 既定機能を使うか: bool, 機能一覧: &'static [&'static str]
) -> 機能台帳の項 {
    機能台帳の項 {
        依存名,
        既定機能を使うか,
        機能一覧,
    }
}
