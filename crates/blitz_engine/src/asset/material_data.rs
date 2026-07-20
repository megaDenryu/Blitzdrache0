//! マテリアルデータ: glTFから読み込んだPBR metallic-roughnessパラメータ(判断23)。
//! テクスチャが無い場合は`None`(無言のデフォルト適用禁止。呼び出し側=blitz_appが
//! 明示的既定素材を適用する)。

use super::texture_data::テクスチャデータ;

/// ベースカラー・metallicRoughness・法線マップの3テクスチャと、それぞれに
/// 掛け合わせる係数。テクスチャ無しは`None`(白テクスチャデータを作る等で
/// 呼び出し側が補う)。
#[derive(Debug, Clone, PartialEq)]
pub struct マテリアルデータ {
    pub ベースカラー: Option<テクスチャデータ>,
    /// glTF規約: G成分=粗さ、B成分=金属度。
    pub 金属粗さ: Option<テクスチャデータ>,
    pub 法線マップ: Option<テクスチャデータ>,
    pub ベースカラー係数: [f32; 4],
    pub 金属度係数: f32,
    pub 粗さ係数: f32,
}
