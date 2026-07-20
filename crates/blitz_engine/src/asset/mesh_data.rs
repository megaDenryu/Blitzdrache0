//! メッシュデータ: glTFの1プリミティブから取り出した頂点一覧・インデックス一覧。

use super::vertex_attribute::メッシュ頂点属性;

/// 頂点一覧とインデックス一覧の組。
#[derive(Debug, Clone, PartialEq)]
pub struct メッシュデータ {
    pub 頂点一覧: Vec<メッシュ頂点属性>,
    pub インデックス一覧: Vec<u32>,
}
