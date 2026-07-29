//! 大気媒体方針の検証の共通の器。検査そのものと、検査が使う方針の変形の組み立ては子モジュールが持ち、
//! ここには全検査が使う長さの組み立てだけを置く。

#![allow(clippy::unwrap_used)]

mod component_variants;
mod policy_value_tests;
mod policy_variants;
mod static_key_tests;

use blitz_math::メートル;

fn m(値: f32) -> メートル {
    メートル::生成する(値)
}
