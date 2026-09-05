//! 型ごとの分量の上限を、モジュールの根を共有する区画へ分けて持つ。1つの表が1つの区画の現状を写す。
//!
//! 区画で分けるのは、根からの相対パスで綴りを短くし、1行を1行へ収めるためである。全体のパスを毎行へ
//! 書くと整形が折り返し、1つの表が1ファイル100行の原則を超える。

mod blitz_app;
mod blitz_asset_compiler;
mod blitz_collision;
mod blitz_engine;
mod blitz_math;
mod blitz_render;
mod blitz_sim;
mod editor_server;
mod other_crates;
mod xtask;

use super::区画の一覧;

pub fn 全区画() -> Vec<区画の一覧> {
    vec![
        blitz_app::一覧(),
        blitz_asset_compiler::一覧(),
        blitz_collision::一覧(),
        blitz_engine::一覧(),
        blitz_math::一覧(),
        blitz_render::一覧(),
        blitz_sim::一覧(),
        editor_server::一覧(),
        other_crates::一覧(),
        xtask::一覧(),
    ]
}
