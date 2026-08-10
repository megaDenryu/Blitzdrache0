//! 倍精度で組み立てた量を単精度へ狭める境界。受け取るのは倍精度の値、返すのは単精度である。
//!
//! 標準ライブラリは倍精度から単精度への型付き変換を持たず、数値の`as`キャストは規約で禁じているため、
//! glamのベクタ縮小を1成分に使う(`crates/blitz_render/src/local_visibility/narrowing.rs`と
//! `crates/blitz_render/src/atmosphere/narrowing.rs`が同じ手を採る)。
//!
//! 添字と分割数の割り算・奥行きの指数の刻みが倍精度を通るのは、どちらも単精度では刻みが粗くセルの境界が
//! 隣のセルへずれうるためである。単精度へ落ちるのはこの1箇所だけであり、丸めの入る場所が1つに定まる。

use glam::DVec3;

pub(super) fn 単精度へ狭める(値: f64) -> f32 {
    DVec3::splat(値).as_vec3().x
}
