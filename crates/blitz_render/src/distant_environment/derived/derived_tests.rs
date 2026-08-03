//! 派生表現の数学のユニットテストを所有するモジュール。担うのは、GPUを起動せずに確かめられる性質
//! (定数環境の解析解・標本数を上げたときの収束・写像の往復・値域)を関心事ごとに束ねることだけである。
//!
//! GPUと突き合わせる検査は`cargo xtask distant-environment-derived`が別に持つ。こちらが正本の性質を、
//! あちらが写しの一致を見る。

#![allow(clippy::unwrap_used)]

mod brdf_tests;
mod constant_environment_tests;
mod directional_environment_tests;
mod resolution_tests;
mod texel_lookup_tests;

use super::cube_side::立方体画像の一辺;
use super::environment_content::遠方環境の内容;

/// 全テクセルが同じ放射輝度を持つ環境。定数環境の解析解を確かめる各テストが使う。
fn 定数の環境(一辺: u32, 放射輝度: [f64; 3]) -> 遠方環境の内容 {
    let 一辺 = 立方体画像の一辺::生成する(一辺).unwrap();
    遠方環境の内容::生成する(一辺, vec![放射輝度; 一辺.全テクセル数()]).unwrap()
}
