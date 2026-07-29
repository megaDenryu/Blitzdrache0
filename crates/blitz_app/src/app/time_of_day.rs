//! 時間帯の配線。担当するのは「世界の空方針を決め、その方針と時刻からレンダラーへ渡す空入力を作る」ことである。
//!
//! 空を持つ世界だけが時刻を読む。空を持たない世界はこの配線を1度も通らず、絵は世界が決めた固定の光のままである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「段階導入」

mod scene_policy;
#[cfg(test)]
mod scene_policy_tests;
mod sky_input;

pub(super) use scene_policy::{世界の空方針を決める, 空を描くか};
pub(super) use sky_input::既定時刻の空入力を作る;
