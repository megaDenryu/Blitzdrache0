//! アセット層: glTF読込・画像デコード・安定ID・カタログ。
//!
//! 注意: gltf/imageクレートに依存してよいのはこのモジュール配下だけ
//! （README「利用ライブラリ」)。公開APIに両クレートの型を一切露出しない。

mod animation_clip;
mod catalog;
mod error;
mod id;
mod interpolation_kind;
mod joint;
mod joint_channel;
mod keyframe_channel;
mod loader;
mod material_data;
mod mesh_data;
mod scene_data;
mod skin_data;
mod skin_vertex_attribute;
mod static_trs;
mod texture_data;
mod vertex_attribute;

pub use animation_clip::アニメーションクリップ;
pub use catalog::カタログ;
pub use error::アセットエラー;
pub use id::{アセットID, アセットIDエラー};
pub use interpolation_kind::補間種別;
pub use joint::ジョイント;
pub use joint_channel::ジョイントアニメーションチャンネル;
pub use keyframe_channel::チャンネル;
pub use loader::シーンを読み込む;
pub use material_data::マテリアルデータ;
pub use mesh_data::メッシュデータ;
pub use scene_data::シーンデータ;
pub use skin_data::スキンデータ;
pub use skin_vertex_attribute::スキン頂点属性;
pub use static_trs::静的TRS;
pub use texture_data::{テクスチャデータ, 法線マップ既定テクスチャを作る, 白テクスチャデータを作る};
pub use vertex_attribute::メッシュ頂点属性;
