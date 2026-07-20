//! 開発用UIメッシュ1つぶん(頂点・インデックス・テクスチャ・シザー矩形)。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断33」。

use crate::ui_scissor::UIシザー矩形px;
use crate::ui_texture_id::UIテクスチャID;
use crate::ui_vertex::UI頂点;

/// egui1メッシュぶんの描画データ。フレームごとに毎回作り直す(即時モード)。
#[derive(Debug, Clone, PartialEq)]
pub struct UIメッシュ {
    pub 頂点一覧: Vec<UI頂点>,
    pub インデックス一覧: Vec<u32>,
    pub テクスチャid: UIテクスチャID,
    pub シザー矩形px: UIシザー矩形px,
}
