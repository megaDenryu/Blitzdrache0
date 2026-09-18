//! 世界実行の契約: `アプリ`と`世界実行`の両側が読む値の型を1つの面として置く。
//! 配線の型(`ゲーム配線`・各ゲームの配線・`ゲーム状態の台帳`)はここに現れず、`アプリ`からは呼べない。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断12」の「値の型は共有し、配線の型は閉じる」。
//!
//! 固定刻みの入口が受け渡す`刻み入力`と`刻み結果`、描画機会の抽出の入口が作る`描画供給`はこの面が定義する。
//! 計器・要約・世界の形を尋ねる口の実装エラー・読込済みチャンクの形の出どころは、それらを作る型と同じ場所で定義されており、
//! この面はその定義をそのまま見せる。

mod camera_input;
mod display_distance;
mod draw_supply;
mod height_field;
mod tick_input;
mod tick_result;

pub(crate) use camera_input::この描画のカメラの入力;
pub(crate) use display_distance::表示距離の指示;
pub(crate) use draw_supply::描画供給;
pub(crate) use height_field::ゲーム用高さ場;
pub(crate) use tick_input::刻み入力;
pub(crate) use tick_result::刻み結果;

pub(crate) use super::instrument::{カメラの計器, 直前の刻みの移動, 直前の描画のカメラ, 移動とカメラの計器, 移動の計器};
pub(crate) use super::query_count_distribution::問い合わせ件数の要約;
pub(crate) use super::shape_version_record::刻みが見た世界の形の版の要約;
pub(crate) use super::step_time_by_query_count::件数別の刻みの所要時間の要約;
pub(crate) use super::step_time_distribution::刻みの所要時間の要約;
pub(crate) use super::summary::ゲーム進行の要約;
pub(crate) use super::world_shape_port::{世界の形を尋ねる口の実装エラー, 読込済みチャンクの形の出どころ};
