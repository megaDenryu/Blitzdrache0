//! フレーム型: 座標系（空間）を幻影型で区別する位置・変換。
//! 参照: `_doc/計画/ユビキタス言語.md`「フレーム型」「幻影型」。

mod position;
mod rotation;
mod space;
mod transform;
mod transform_construct;
mod trs;
#[cfg(test)]
mod trs_tests;

pub use position::位置;
pub use rotation::{クォータニオン, クォータニオンエラー};
pub use space::{クリップ, ビュー, ローカル, ワールド, 光源クリップ, 空間};
pub use transform::変換;
