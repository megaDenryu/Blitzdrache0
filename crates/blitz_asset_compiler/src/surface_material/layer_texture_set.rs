//! 地表層テクスチャ集: 地表材質の層に貼るタイル画像を読み、世界に1つだけの共有アセットとして焼く経路。
//! 層がどの材質名を参照するかはエディターのマテリアル台帳が決め、名前から索引への解決はこの経路の1回で終える。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`

mod bake_choice;
mod compile;
mod error;
mod layer_assignment;
mod tile_directory;

pub use bake_choice::地表層テクスチャ集の焼き方;
pub use compile::{コンパイル済み地表層テクスチャ集, 地表層テクスチャ集アセットをコンパイルする};
pub use error::地表層テクスチャ集コンパイルエラー;
pub use layer_assignment::地表材質の層割当;
pub use tile_directory::地表層タイルの置き場;
