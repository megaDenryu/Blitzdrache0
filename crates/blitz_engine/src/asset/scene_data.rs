//! シーンデータ: glTFから読み込んだ最初のメッシュとそのマテリアル。

use std::path::PathBuf;

use super::animation_clip::アニメーションクリップ;
use super::material_data::マテリアルデータ;
use super::mesh_data::メッシュデータ;
use super::skin_data::スキンデータ;

/// メッシュと、そのPBRマテリアル(判断23)。
///
/// `参照ファイル一覧` はローダが実際に読んだファイル群(主ファイル・外部バッファ・
/// 外部画像)。アセットホットリロードのmtime監視対象として使う。
///
/// `スキン`と`アニメーション一覧`はスキン付きメッシュのみ値を持つ(判断42)。
/// スキン無しシーンは`スキン: None`・`アニメーション一覧: 空のVec`で、既存の静的シーンの
/// 読込結果は無変更のまま通る。
#[derive(Debug, Clone, PartialEq)]
pub struct シーンデータ {
    pub メッシュ: メッシュデータ,
    pub マテリアル: マテリアルデータ,
    pub 参照ファイル一覧: Vec<PathBuf>,
    pub スキン: Option<スキンデータ>,
    pub アニメーション一覧: Vec<アニメーションクリップ>,
}
