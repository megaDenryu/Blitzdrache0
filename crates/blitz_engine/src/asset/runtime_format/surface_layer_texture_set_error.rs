//! 地表層テクスチャ集の実行時形式の読み書きで返す型付きエラー。共通ヘッダーと境界検査の失敗と、
//! テクスチャ集そのものの検証の失敗と、タイルのテクスチャの検証の失敗の3つを畳む。
//!
//! 種別ごとの語彙へ置くのは、高さ場の実行時形式のエラーと同じ理由である
//! (参照: `height_field_error.rs`)。

use super::アセット実行時形式エラー;
use crate::asset::texture_storage::格納済みテクスチャエラー;
use crate::surface_layer_textures::地表層テクスチャ集エラー;

#[derive(Debug, thiserror::Error)]
pub enum 地表層テクスチャ集実行時形式エラー {
    #[error(transparent)]
    共通形式の失敗(#[from] アセット実行時形式エラー),
    #[error(transparent)]
    テクスチャ集の内容の失敗(#[from] 地表層テクスチャ集エラー),
    #[error(transparent)]
    タイルのテクスチャの失敗(#[from] 格納済みテクスチャエラー),
    #[error("地表層テクスチャ集のタイルがベースカラーのテクスチャを持たない: 材質名={材質名}")]
    タイルにベースカラーが無い { 材質名: String },
    #[error("地表層テクスチャ集の材質名がUTF-8として読めない")]
    材質名が文字列として読めない,
}
