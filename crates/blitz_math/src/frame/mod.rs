//! フレーム型: 座標系（空間）を幻影型で区別する位置・変換。
//! 参照: `_doc/計画/ユビキタス言語.md`「フレーム型」「幻影型」。

mod camera_relative_position;
mod direction;
mod direction_error;
#[cfg(test)]
mod direction_tests;
mod displacement;
mod global_world_position;
mod pixel_jitter;
mod position;
mod position_displacement;
mod rotation;
mod rotation_compose;
mod rotation_error;
mod rotation_from_axes;
#[cfg(test)]
mod rotation_from_axes_tests;
mod space;
mod transform;
mod transform_construct;
mod transform_construct_cube_face;
mod trs;
#[cfg(test)]
mod trs_tests;

pub use camera_relative_position::{カメラ相対位置, 座標変換エラー};
pub use direction::方向;
pub use direction_error::方向エラー;
pub use displacement::変位;
pub use global_world_position::大域ワールド位置;
pub use pixel_jitter::{画素内ずらし, 画素内ずらしエラー};
pub use position::位置;
pub use rotation::クォータニオン;
pub use rotation_error::クォータニオンエラー;
pub use space::{クリップ, ビュー, ローカル, ワールド, 光源クリップ, 点光源の面クリップ, 空間, 部品ローカル};
pub use transform::変換;
