//! 未是正の自由関数の一覧を、モジュールの根を共有する区画へ分けて持つ。1つの表が1つの区画の現状を写す。
//!
//! 区画で分けるのは、1ファイル100行の原則を守るためだけでなく、是正が区画の単位で進むためである。
//! コンポジションルートの多段化は`crates/blitz_app/src/app`の区画をまとめて消す。

mod blitz_app_app;
mod blitz_app_other;
mod blitz_render_other;
mod blitz_render_renderer;
mod blitz_render_vulkan;
mod other_crates;
mod xtask;

use super::区画の一覧;

pub fn 全区画() -> Vec<区画の一覧> {
    vec![
        blitz_render_vulkan::一覧(),
        blitz_render_renderer::一覧(),
        blitz_render_other::一覧(),
        blitz_app_app::一覧(),
        blitz_app_other::一覧(),
        xtask::一覧(),
        other_crates::一覧(),
    ]
}
