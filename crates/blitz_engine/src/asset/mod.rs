//! アセット層: 実行時形式、安定ID、カタログ、シーンデータ。

mod animation_clip;
mod catalog;
mod chunk_id;
mod id;
mod interpolation_kind;
mod joint;
mod joint_channel;
mod keyframe_channel;
mod material_data;
mod mesh_data;
mod pbr_material_data;
mod render_object_data;
mod render_object_id;
mod runtime_format;
#[cfg(test)]
mod runtime_format_tests;
mod runtime_load_error;
mod runtime_loader;
#[cfg(test)]
mod runtime_loader_tests;
#[cfg(test)]
mod runtime_scene_tests;
mod scene_data;
mod skin_data;
mod skin_vertex_attribute;
mod static_trs;
mod texture_data;
mod vertex_attribute;

pub use animation_clip::アニメーションクリップ;
pub use catalog::カタログ;
pub use chunk_id::チャンクID;
pub use id::{アセットID, アセットIDエラー};
pub use interpolation_kind::補間種別;
pub use joint::ジョイント;
pub use joint_channel::ジョイントアニメーションチャンネル;
pub use keyframe_channel::チャンネル;
pub use material_data::マテリアルデータ;
pub use mesh_data::メッシュデータ;
pub use pbr_material_data::金属粗さPBRデータ;
pub use render_object_data::描画対象データ;
pub use render_object_id::描画対象ID;
pub use runtime_format::{
    アセット実行時形式エラー, アセット形式版, シーンを実行時形式へ格納する, 実行時アセット, 実行時アセットを格納する, 実行時アセットを開く,
    実行時アセット種別, 実行時形式からシーンを読む,
};
pub use runtime_load_error::実行時シーン読込エラー;
pub use runtime_loader::実行時シーンを読み込む;
pub use scene_data::シーンデータ;
pub use skin_data::スキンデータ;
pub use skin_vertex_attribute::スキン頂点属性;
pub use static_trs::静的TRS;
pub use texture_data::{テクスチャデータ, 法線マップ既定テクスチャを作る, 白テクスチャデータを作る};
pub use vertex_attribute::メッシュ頂点属性;
