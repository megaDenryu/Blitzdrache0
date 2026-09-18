//! 世界実行: ゲーム世界を進める入口を時間意味(起動時・固定刻み・描画機会の更新・描画機会の抽出・必要時・終了時)ごとに
//! 限定してアプリへ貸し、2つのゲーム(キツネの場所巡り・歩くだけ)が共有する状態を所有する型。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断12」。
//!
//! 共有される状態をこの型が持つため、ゲームの種類で振り分ける`match`は、ゲーム固有の状態(進行段階・道順・移動状態)を
//! 読む口だけに残る。ゲームの種類ごとの配線は子モジュール`game`が持ち、`アプリ`からは見えない。
//! 両側が読む値の型は`contract`が1つの面として持つ。
//!
//! 遊ばない起動でも共有される状態は在る。台帳は動く個体を1体も持たず、高さ場は持たない世界、カメラはプレイヤー用の初期値、
//! 移動の記録はまだ刻んでいない記録である。遊ばない起動が刻みも操作の確定もカメラの操作も1つも行わないことは、
//! ゲームの種類の振り分け(`ゲーム配線::ゲームを遊ぶか`)が各入口の先頭で決める。
//!
//! 入口は時間意味ごとのファイルにある。起動時は`create`、固定刻みは`tick`、描画機会の更新と抽出は`draw`、
//! 起動時と必要時の高さ場と動く個体の宣言は`height_field`、終了時の要約は`draw`と同じ抽出の面に置く。

mod camera_system;
mod camera_wiring;
#[cfg(test)]
mod camera_wiring_test_ports;
#[cfg(test)]
mod camera_wiring_tests;
pub(crate) mod contract;
mod create;
mod draw;
mod entity_id;
mod entity_ledger;
mod game;
mod ground_height;
mod height_field;
mod instrument;
mod movement_record;
#[cfg(test)]
mod no_game_launch_tests;
mod query_count_distribution;
mod shape_version_record;
mod step_seconds;
mod step_time_by_query_count;
mod step_time_distribution;
#[cfg(test)]
mod step_time_distribution_tests;
mod summary;
mod tick;
#[cfg(test)]
mod tick_confirmation_tests;
#[cfg(test)]
mod tick_input_tests;
mod tick_stage;
mod world_shape_port;
#[cfg(test)]
mod world_shape_port_tests;

pub(crate) use step_seconds::ゲーム更新の一刻みの秒;

use blitz_engine::height_field::高さ場の読み口;

use camera_wiring::プレイヤーカメラの配線;
use entity_ledger::ゲーム状態の台帳;
use movement_record::移動の観測の記録;

use crate::input::ゲーム操作の適用方針;

/// `ゲーム配線`はどのゲームを回すかの振り分けとゲーム固有の状態だけを持つ。
/// `操作の適用方針`は実ウィンドウから届く操作をフレーム数の決まった実行で遮断する方針であり、ゲームの種類によらない。
/// `台帳`は動く個体の描画先と供給値、`高さ場の読み口`は世界の高さ、`カメラ`はプレイヤーカメラの系統と遮蔽の復帰、
/// `移動の記録`は刻みの観測の累計であり、どれも2つのゲームが同じ1つを使う。
pub(crate) struct 世界実行 {
    ゲーム配線: game::ゲーム配線,
    操作の適用方針: ゲーム操作の適用方針,
    台帳: ゲーム状態の台帳,
    高さ場の読み口: 高さ場の読み口,
    カメラ: プレイヤーカメラの配線,
    移動の記録: 移動の観測の記録,
}
