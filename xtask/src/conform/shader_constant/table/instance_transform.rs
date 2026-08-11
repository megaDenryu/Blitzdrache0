//! 個体1件がGPUで占める変換のバイト数について、見積を持つblitz_engineと実際の配置を持つblitz_renderの対応。
//! シェーダーの写しではなくクレート同士の対であり、値がずれても実行時に例外にならない。
//! ずれると常駐メモリの見積だけが古い数字のまま残り、収容の判断が実確保から離れる。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_render/src/vulkan/instance_transform/bytes.rs",
    正本の前置き: "pub(crate) const バイト長: usize = ",
    写しパス: "crates/blitz_engine/src/asset/runtime_format/scene/mesh_layout.rs",
    写しの前置き: "pub(crate) const 個体変換長: usize = ",
}];
