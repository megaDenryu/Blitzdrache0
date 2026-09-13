//! 弱圧縮性SPHによる少量3D流体のCPU参照計算(M10判断59)。

mod density;
mod error;
mod kernel;
mod particle;
mod spec;
mod step;
#[cfg(test)]
mod step_tests;

pub use density::粒子法流体の密度を計算する;
pub use error::粒子法流体仕様エラー;
pub use particle::流体粒子;
pub use spec::粒子法流体仕様;
pub use step::粒子法流体を一ステップ進める;
