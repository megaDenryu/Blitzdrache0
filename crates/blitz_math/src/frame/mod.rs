//! フレーム型: 座標系（空間）を幻影型で区別する位置・変換。
//! 参照: `_doc/計画/ユビキタス言語.md`「フレーム型」「幻影型」。

mod position;
mod space;
mod transform;
mod transform_construct;

pub use position::位置;
pub use space::{クリップ, ビュー, ローカル, ワールド, 空間};
pub use transform::変換;
