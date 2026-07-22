//! glTFのPBR metallic-roughnessパラメータ。

use super::texture_data::テクスチャデータ;

#[derive(Debug, Clone, PartialEq)]
pub struct 金属粗さPBRデータ {
    pub ベースカラー: Option<テクスチャデータ>,
    /// glTF規約ではG成分が粗さ、B成分が金属度を表す。
    pub 金属粗さ: Option<テクスチャデータ>,
    pub 法線マップ: Option<テクスチャデータ>,
    pub ベースカラー係数: [f32; 4],
    pub 金属度係数: f32,
    pub 粗さ係数: f32,
}
