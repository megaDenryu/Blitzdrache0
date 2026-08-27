//! 高さ場: 世界の高さを実行中に参照できる形で持つアセットと、その問い合わせの口。
//! アセットコンパイラの入力にとどまる高さ格子とは別物であり、こちらは実行時形式に現れて実行中の問い合わせに答える。
//! 実行時形式のバイト列との往復は`asset/runtime_format/height_field_v1`が実装し、その入口をこのモジュールから見せ直す。
//! 語彙が諸元・問い合わせ結果・読み口・3種類の失敗と多いため、クレート直下へ平坦に再エクスポートせずモジュールごと公開する。
//! 参照: `_doc/計画/ユビキタス言語.md`「高さ場」、`_doc/設計/ゲーム制作アーキテクチャ.md`「判断7: 地図の正本を持ち、生成は2系統に分ける」

mod capsule_contact;
mod capsule_query_result;
#[cfg(test)]
mod capsule_tests;
mod cell_position;
mod distant_stable_id;
mod error;
mod field;
mod field_capsule;
mod field_corner_source;
mod field_ground_surface;
mod field_segment;
mod grid_index;
mod grid_origin_frame;
mod ground_surface;
mod ground_surface_query_result;
#[cfg(test)]
mod ground_surface_tests;
#[cfg(test)]
pub(crate) mod height_field_tests;
mod load_error;
mod query_result;
mod reader;
mod segment_hit;
mod segment_query_result;
#[cfg(test)]
mod segment_tests;
mod specification;
mod stable_id;
mod triangle_location;

pub use blitz_collision::height_field::{
    カプセルの掃引の問い合わせエラー, 升目の三角形, 升目の隅, 地表に触れた特徴, 掃引で動けた割合, 線分の問い合わせエラー, 線分の媒介変数,
};
pub use capsule_contact::{カプセルと地表の接触, 掃引したカプセルが最初に触れる地表};
pub use capsule_query_result::掃引したカプセルが最初に触れる地表の問い合わせ結果;
pub use distant_stable_id::世界の遠景地形の安定IDの綴り;
pub use error::高さ場エラー;
pub use field::高さ場;
pub use ground_surface::世界の地表の面;
pub use ground_surface_query_result::地表の面の問い合わせ結果;
pub use load_error::高さ場読込エラー;
pub use query_result::地表高さの問い合わせ結果;
pub use reader::高さ場の読み口;
pub use segment_hit::{線分が最初に当たる地表, 線分と地表の当たり};
pub use segment_query_result::線分が最初に当たる地表の問い合わせ結果;
pub use specification::高さ場諸元;
pub use stable_id::世界の高さ場の安定IDの綴り;
pub use triangle_location::地表の三角形の所在;

pub use crate::asset::{実行時形式から高さ場を読む, 高さ場を実行時形式へ格納する, 高さ場実行時形式エラー};
