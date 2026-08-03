//! 時間帯の配線。担当するのは「世界の空方針を決め、ゲーム時計の世界時刻から、そのフレームの空入力・ライティング入力・
//! 露出倍率を作る」ことである。
//!
//! 空を持つ世界だけが時刻を読む。空を持たない世界はこの配線を1度も通らず、絵は世界が決めた固定の光のままである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「段階導入」

mod aerial_input;
mod atmosphere_input;
mod atmosphere_update;
#[cfg(test)]
mod atmosphere_update_tests;
mod clock;
mod distant_environment_update;
#[cfg(test)]
mod distant_environment_update_tests;
mod reproduction_condition;
mod scene_policy;
#[cfg(test)]
mod scene_policy_tests;
mod sky_input;
mod sun_disk_override;
mod wiring;

pub(crate) use distant_environment_update::遠方環境更新判定;
pub(crate) use reproduction_condition::空の再現条件;
pub(in crate::app) use wiring::{天空配線, 焼き上げ入力の組};
