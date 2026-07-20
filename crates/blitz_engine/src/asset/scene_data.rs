//! シーンデータ: glTFから読み込んだ最初のメッシュとそのベースカラーテクスチャ。

use std::path::PathBuf;

use super::mesh_data::メッシュデータ;
use super::texture_data::テクスチャデータ;

/// メッシュと、あれば付随するベースカラーテクスチャ。
///
/// `参照ファイル一覧` はローダが実際に読んだファイル群(主ファイル・外部バッファ・
/// 外部画像)。アセットホットリロードのmtime監視対象として使う。
#[derive(Debug, Clone, PartialEq)]
pub struct シーンデータ {
    pub メッシュ: メッシュデータ,
    pub ベースカラー: Option<テクスチャデータ>,
    pub 参照ファイル一覧: Vec<PathBuf>,
}
