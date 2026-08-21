//! 地表材質の塗り分けの層の数。担当するのは、材質テクスチャ役割・材質レコード・シェーダーの3つが同じ数の層を
//! 前提にすることである。
//!
//! 正本は`crates/blitz_engine/src/surface_layer_textures.rs`の同名の定数であり、ここはその写しである。
//! レンダラーがblitz_engineに依存しないため値を持ち直す必要があり、両者の一致は`cargo xtask conform`が突き合わせる。

pub const 地表層の数: usize = 4;
