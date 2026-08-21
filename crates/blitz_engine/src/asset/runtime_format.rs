//! アセット実行時形式の共通ヘッダーと検証済みの読み取り結果。
//! 形式版の数え上げは`asset_version`が、種別の数え上げは`asset_kind`が持つ。

mod asset_kind;
mod asset_version;
mod catalog;
mod chunk_directory_error;
mod chunk_directory_v2;
mod error;
mod header;
mod height_field_error;
mod height_field_v1;
mod material_assignment_error;
mod scene;
mod surface_layer_texture_set_error;
mod surface_layer_texture_set_v1;

pub use asset_kind::実行時アセット種別;
pub use asset_version::アセット形式版;
pub use catalog::{カタログを実行時形式へ格納する, 実行時形式からカタログを読む};
pub use chunk_directory_v2::{チャンク目録を実行時形式へ格納する, 実行時形式からチャンク目録を読む};
pub use error::アセット実行時形式エラー;
pub use header::{実行時アセットを格納する, 実行時アセットを開く};
pub use height_field_error::高さ場実行時形式エラー;
pub use height_field_v1::{実行時形式から高さ場を読む, 高さ場を実行時形式へ格納する};
pub use material_assignment_error::材質割当エラー;
pub(crate) use scene::mesh_layout;
pub use scene::{シーンを実行時形式へ格納する, 実行時形式からシーンを読む};
pub use surface_layer_texture_set_error::地表層テクスチャ集実行時形式エラー;
pub use surface_layer_texture_set_v1::{
    地表層テクスチャ集を実行時形式へ格納する, 実行時形式から地表層テクスチャ集を読む
};

pub(super) const ヘッダー長: usize = 24;
pub(super) const 固定識別値: [u8; 8] = *b"BLITZAST";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 実行時アセット<'a> {
    pub 形式版: アセット形式版,
    pub 種別: 実行時アセット種別,
    pub 内容: &'a [u8],
}
