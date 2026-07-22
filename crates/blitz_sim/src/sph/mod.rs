//! 弱圧縮性SPHによる少量3D流体のCPU参照計算(M10判断59)。

mod density;
mod error;
mod kernel;
mod particle;
mod spec;
mod step;
#[cfg(test)]
mod step_tests;

pub use density::sph密度を計算する;
pub use error::Sph仕様エラー;
pub use particle::流体粒子;
pub use spec::Sph仕様;
pub use step::sphを一ステップ進める;
