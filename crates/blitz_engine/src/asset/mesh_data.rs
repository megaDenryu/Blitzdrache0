//! メッシュデータ: glTFの1プリミティブから取り出した頂点一覧・インデックス一覧。

use super::skin_vertex_attribute::スキン頂点属性;
use super::vertex_attribute::メッシュ頂点属性;

/// 頂点一覧とインデックス一覧の組。
///
/// `スキン頂点属性一覧`はスキン付きメッシュのみ`Some`で、`頂点一覧`と同じ長さになる
/// (判断42)。スキン無しメッシュは`None`のままで、既存の静的シーンは無変更で通る。
#[derive(Debug, Clone, PartialEq)]
pub struct メッシュデータ {
    pub 頂点一覧: Vec<メッシュ頂点属性>,
    pub インデックス一覧: Vec<u32>,
    pub スキン頂点属性一覧: Option<Vec<スキン頂点属性>>,
}
