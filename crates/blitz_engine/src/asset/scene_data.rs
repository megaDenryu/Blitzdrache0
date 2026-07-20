//! シーンデータ: glTFから読み込んだ最初のメッシュとそのベースカラーテクスチャ。

use super::mesh_data::メッシュデータ;
use super::texture_data::テクスチャデータ;

/// メッシュと、あれば付随するベースカラーテクスチャ。
#[derive(Debug, Clone, PartialEq)]
pub struct シーンデータ {
    pub メッシュ: メッシュデータ,
    pub ベースカラー: Option<テクスチャデータ>,
}
