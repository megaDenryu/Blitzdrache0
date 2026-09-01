//! 時間帯の配線。担当するのは「世界の空方針を決め、ゲーム時計の世界時刻から、そのフレームの空入力・ライティング入力・
//! 露出倍率を作る」ことである。
//!
//! 空を持つ世界だけが時刻を読む。空を持たない世界はこの配線を1度も通らず、絵は世界が決めた固定の光のままである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「段階導入」

mod aerial_distance_record;
mod aerial_frustum;
#[cfg(test)]
mod aerial_frustum_tests;
mod aerial_input;
mod atmosphere_input;
mod atmosphere_update;
#[cfg(test)]
mod atmosphere_update_tests;
#[cfg(test)]
mod bake_commit_tests;
mod clock;
mod distant_environment_input;
mod distant_environment_key_record;
mod distant_environment_update;
#[cfg(test)]
mod distant_environment_update_tests;
mod exposure_elapsed;
#[cfg(test)]
mod indirect_lighting_chain_tests;
mod interval_override;
mod reproduction_condition;
pub(in crate::app) mod scene_policy;
#[cfg(test)]
mod scene_policy_tests;
mod sky_input;
mod step_scan;
mod sun_disk_override;
mod sun_zenith_interval_record;
mod thinning;
#[cfg(test)]
mod thinning_commit_tests;
mod wiring;

pub(crate) use aerial_distance_record::空中遠近の最遠距離の記録;
pub(crate) use distant_environment_key_record::遠方環境の鍵の記録;
pub(crate) use distant_environment_update::遠方環境更新判定;
pub(crate) use reproduction_condition::空の再現条件;
pub(crate) use step_scan::段差走査の指定;
pub(crate) use sun_zenith_interval_record::太陽天頂区間の記録;
pub(in crate::app) use wiring::{天空配線, 焼き上げ入力の組, 生成材料};
