//! 接合点の語彙。部品どうしを噛み合わせる位置・姿勢・接合面の寸法を、部品のローカル座標系で表す型を集める。
//!
//! この束が知ってよいのは`blitz_math`だけである。部品もカタログも知らない。逆向きの参照を許すと、
//! 接合面の姿勢を作るだけの計算がカタログを引きずる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「接合点の契約」

mod error;
mod face_pose;
mod face_position;
mod face_size;
mod joint_kind;
mod joint_kind_pairing;
mod joint_kind_style;
mod joint_name;
mod joint_point;
mod mating_style;
mod normal_offset;
mod positive_length;

#[cfg(test)]
mod joint_kind_pairing_tests;
#[cfg(test)]
mod joint_kind_style_tests;
#[cfg(test)]
mod joint_tests;

pub use error::接合点エラー;
pub use face_pose::接合面の姿勢;
pub use face_position::接合面上の位置;
pub use face_size::接合面の形と寸法;
pub use joint_kind::{全接合種別, 接合種別};
pub use joint_name::接合点名;
pub use joint_point::接合点;
pub use mating_style::{包含の役割, 接合の様式};
pub use normal_offset::法線方向のずらし;
pub use positive_length::正で有限な長さ;
