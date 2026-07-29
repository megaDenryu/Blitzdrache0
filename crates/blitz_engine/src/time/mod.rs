//! 時刻の型: 永続世界の絶対時刻(世界時刻)と一日の中で循環する時刻(一日内時刻)を別の型で表し、
//! 進行・停止・倍率はゲーム時計が所有する。単なる0〜24の数にしないのは、同一時刻が同一入力へ写ることを
//! 型で保証し、日数・季節を後から入れる余地を閉じないためである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「時刻の型」

mod day_length;
mod day_time;
mod game_clock;
mod time_error;
mod time_scale;
mod world_time;

#[cfg(test)]
mod game_clock_tests;
#[cfg(test)]
mod world_time_tests;

pub use day_length::一日の長さ;
pub use day_time::一日内時刻;
pub use game_clock::{ゲーム時計, 時計進行状態};
pub use time_error::時刻エラー;
pub use time_scale::時間倍率;
pub use world_time::世界時刻;
