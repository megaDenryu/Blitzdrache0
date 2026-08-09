//! アセット層: 実行時形式、安定ID、カタログ、シーンデータ。

mod animation_clip;
mod asset_metadata;
mod catalog;
mod catalog_entry;
mod catalog_load_error;
mod catalog_loader;
mod draw_shape;
mod id;
mod instance;
mod interpolation_kind;
mod joint;
mod joint_channel;
mod keyframe_channel;
mod material_data;
mod material_set;
mod material_set_error;
mod material_slot_assignment;
mod material_slot_id;
mod mesh_data;
mod mesh_primitive;
mod pbr_material_data;
mod render_object_data;
mod render_object_id;
#[cfg(test)]
mod runtime_catalog_error_tests;
#[cfg(test)]
mod runtime_catalog_tests;
#[cfg(test)]
mod runtime_chunk_directory_tests;
mod runtime_format;
#[cfg(test)]
mod runtime_format_tests;
mod runtime_load_error;
mod runtime_loader;
#[cfg(test)]
mod runtime_loader_tests;
#[cfg(test)]
mod runtime_scene_instance_tests;
#[cfg(test)]
pub(crate) mod runtime_scene_multi_material_tests;
#[cfg(test)]
mod runtime_scene_terrain_tests;
#[cfg(test)]
pub(crate) mod runtime_scene_tests;
mod scene_data;
mod skin_data;
mod skin_vertex_attribute;
mod static_trs;
mod terrain_lod_meshes;
mod texture_data;
/// テクスチャの格納の語彙は形式・ブロックの格子・バイト数の算術のエラーと数が多く、平坦な再エクスポートへ混ぜると呼び出し側が出どころを追えなくなるため、モジュールごと公開する。
pub mod texture_storage;
mod vertex_attribute;

pub use animation_clip::アニメーションクリップ;
pub use asset_metadata::アセットメタデータ;
pub use catalog::カタログ;
pub use catalog_entry::カタログ項目;
pub use catalog_load_error::実行時カタログ読込エラー;
pub use catalog_loader::実行時カタログを読み込む;
pub use draw_shape::描画形状;
pub use id::{アセットID, アセットIDエラー};
pub use instance::{
    インスタンス群, インスタンス群エラー, チャンクの基準原点からの許容メートル, 個体配置, 原型, 境界球, 群境界, 軸平行包囲領域
};
pub use interpolation_kind::補間種別;
pub use joint::ジョイント;
pub use joint_channel::ジョイントアニメーションチャンネル;
pub use keyframe_channel::チャンネル;
pub use material_data::マテリアルデータ;
pub use material_set::材質集合;
pub use material_set_error::材質集合エラー;
pub use material_slot_assignment::材質スロット割当;
pub use material_slot_id::材質スロットID;
pub use mesh_data::メッシュデータ;
pub use mesh_primitive::メッシュプリミティブ;
pub use pbr_material_data::{材質特徴集合, 金属粗さPBRデータ};
pub use render_object_data::描画対象データ;
pub use render_object_id::描画対象ID;
pub(crate) use runtime_format::mesh_layout;
pub use runtime_format::{
    アセット実行時形式エラー, アセット形式版, カタログを実行時形式へ格納する, シーンを実行時形式へ格納する, チャンク目録を実行時形式へ格納する,
    実行時アセット, 実行時アセットを格納する, 実行時アセットを開く, 実行時アセット種別, 実行時形式からカタログを読む, 実行時形式からシーンを読む,
    実行時形式からチャンク目録を読む, 材質割当エラー,
};
pub use runtime_load_error::実行時シーン読込エラー;
pub use runtime_loader::実行時シーンを読み込む;
pub(crate) use runtime_loader::実行時シーンファイルを読み込む;
pub use scene_data::シーンデータ;
pub use skin_data::スキンデータ;
pub use skin_vertex_attribute::スキン頂点属性;
pub use static_trs::静的TRS;
pub use terrain_lod_meshes::{地形LODメッシュ群, 地形LODメッシュ群エラー};
pub use texture_data::{テクスチャデータ, 法線マップ既定テクスチャを作る, 白テクスチャデータを作る};
pub use vertex_attribute::メッシュ頂点属性;
