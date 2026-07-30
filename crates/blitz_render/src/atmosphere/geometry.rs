//! 惑星と大気を同心の球として扱う幾何を所有するモジュール。担当するのは、3次元ベクトルの基本演算・視線と球の交差・
//! 球面上に散らした向きの並びである。
//!
//! ここにあるのは媒体の物性を1つも読まない純幾何であり、入力は位置と向きと半径だけである。
//! 積分の手順から幾何を分けるのは、交差の式が閉形式であり、判別式の符号や自己交差の扱いといった
//! 数値の地雷が積分の刻み方とは独立に検査できるためである。

pub(in crate::atmosphere) mod intersect;
pub(in crate::atmosphere) mod sphere_directions;
pub(in crate::atmosphere) mod vector3;
