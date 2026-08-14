//! 高さ場: 世界の高さを実行中に参照できる形で持つアセットと、その問い合わせの口。
//! アセットコンパイラの入力にとどまる高さ格子とは別物であり、こちらは実行時形式に現れて実行中の問い合わせに答える。
//! 実行時形式のバイト列との往復は`asset/runtime_format/height_field_v1`が実装し、その入口をこのモジュールから見せ直す。
//! 語彙が諸元・問い合わせ結果・読み口・3種類の失敗と多いため、クレート直下へ平坦に再エクスポートせずモジュールごと公開する。
//! 参照: `_doc/計画/ユビキタス言語.md`「高さ場」、`_doc/設計/ゲーム制作アーキテクチャ.md`「判断7: 地図の正本を持ち、生成は2系統に分ける」

mod bilinear;
mod distant_stable_id;
mod error;
mod field;
mod grid_index;
#[cfg(test)]
pub(crate) mod height_field_tests;
mod load_error;
mod query_result;
mod reader;
mod specification;
mod stable_id;

pub use distant_stable_id::世界の遠景地形の安定IDの綴り;
pub use error::高さ場エラー;
pub use field::高さ場;
pub use load_error::高さ場読込エラー;
pub use query_result::地表高さの問い合わせ結果;
pub use reader::高さ場の読み口;
pub use specification::高さ場諸元;
pub use stable_id::世界の高さ場の安定IDの綴り;

pub use crate::asset::{実行時形式から高さ場を読む, 高さ場を実行時形式へ格納する, 高さ場実行時形式エラー};
