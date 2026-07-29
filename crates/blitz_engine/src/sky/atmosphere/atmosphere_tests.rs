//! 大気媒体方針の検証の共通の器。検査そのものは2つの子モジュールが持ち、ここには全検査が使う長さの組み立てだけを置く。

#![allow(clippy::unwrap_used)]

mod policy_value_tests;
mod static_key_tests;

use blitz_math::メートル;

fn m(値: f32) -> メートル {
    メートル::生成する(値)
}
