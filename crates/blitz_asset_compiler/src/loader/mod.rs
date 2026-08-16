//! glTF読込の入口。gltf/image型はこのクレートの公開APIに出さない。

mod animation;
mod archetype;
mod archetype_material;
#[cfg(test)]
mod archetype_material_tests;
mod buffer;
mod contract;
mod document;
mod file;
mod fixed_object;
mod joint;
mod material;
mod material_slots;
mod mesh;
mod mesh_bounds;
mod mesh_bounds_reason;
mod part_archetype;
mod skin;
mod static_scene;
mod texture_decode;

pub use archetype::原型ソース;
pub use contract::{
    入力契約を検査するglTFのファイル, 契約指摘, 契約検査概要, 契約検査結果, 検査する契約, 重大度
};
pub use joint::{
    接合点の読み取りの成否, 接合点読み取りエラー, 部品のglTFのファイル, 部品の接合点の読み取り, 部品の読み取りエラー, 部品カタログの読み込みエラー,
    部品カタログの読み込み係,
};
