//! シーンデータ: glTFから読み込んだ最初のメッシュとそのマテリアル。

use std::path::PathBuf;

use super::material_data::マテリアルデータ;
use super::mesh_data::メッシュデータ;

/// メッシュと、そのPBRマテリアル(判断23)。
///
/// `参照ファイル一覧` はローダが実際に読んだファイル群(主ファイル・外部バッファ・
/// 外部画像)。アセットホットリロードのmtime監視対象として使う。
#[derive(Debug, Clone, PartialEq)]
pub struct シーンデータ {
    pub メッシュ: メッシュデータ,
    pub マテリアル: マテリアルデータ,
    pub 参照ファイル一覧: Vec<PathBuf>,
}
