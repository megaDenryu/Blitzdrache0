//! アセット層: glTF読込・画像デコード・安定ID・カタログ。
//!
//! 注意: gltf/imageクレートに依存してよいのはこのモジュール配下だけ
//! （README「利用ライブラリ」)。公開APIに両クレートの型を一切露出しない。

mod catalog;
mod error;
mod id;
mod loader;
mod mesh_data;
mod scene_data;
mod texture_data;
mod vertex_attribute;

pub use catalog::カタログ;
pub use error::アセットエラー;
pub use id::{アセットID, アセットIDエラー};
pub use loader::シーンを読み込む;
pub use mesh_data::メッシュデータ;
pub use scene_data::シーンデータ;
pub use texture_data::{テクスチャデータ, 白テクスチャデータを作る};
pub use vertex_attribute::メッシュ頂点属性;
