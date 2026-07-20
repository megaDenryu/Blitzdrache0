//! 開発用UI(egui)描画用のGPU境界型。フレーム型・単位型を外した生の数値のみを持つ。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断33」。

/// UIメッシュ1頂点ぶんの位置(物理ピクセル)・uv・色。`repr(C)`でVulkanの頂点入力
/// レイアウトと一致させる。色はegui由来のsRGBA(premultiplied alpha)をそのまま運ぶ。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UI頂点 {
    pub 位置px: [f32; 2],
    pub uv: [f32; 2],
    pub 色rgba8: [u8; 4],
}
