//! 空と太陽方針: 世界時刻から天空状態(太陽方向・方向光の色と強度・環境光・露出補正段・空の係数・影の有効性)を
//! 返す純関数と、時刻だけでは決まらない値を持つシーンの方針。同一の世界時刻からは常に同一の天空状態が得られる。
//! 描画への配線は空パスと時間帯統合の段が行い、この層は絵を知らない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「時刻の型」

mod ambient;
mod celestial_policy;
mod daylight_factor;
mod daylight_policy;
mod default_constant;
mod derive;
mod exposure_stop;
mod scene_policy;
mod shadow_validity;
mod sky_coefficients;
mod sky_error;
mod sky_state;
mod sun_direction;
mod sun_light_color;
mod sun_light_intensity;
mod sun_light_policy;
mod sun_position;
mod turbidity;

#[cfg(test)]
mod continuity_tests;
#[cfg(test)]
mod derive_tests;
#[cfg(test)]
mod policy_tests;

pub use ambient::環境光強度;
pub use celestial_policy::天球方針;
pub use daylight_factor::昼係数;
pub use daylight_policy::昼夜方針;
pub use derive::天空状態を導出する;
pub use exposure_stop::露出補正段;
pub use scene_policy::空と太陽の方針;
pub use shadow_validity::影の有効性;
pub use sky_coefficients::空の係数;
pub use sky_error::天空状態エラー;
pub use sky_state::天空状態;
pub use sun_direction::太陽方向;
pub use sun_light_color::太陽光色;
pub use sun_light_intensity::太陽光強度;
pub use sun_light_policy::太陽光方針;
pub use turbidity::濁度;
