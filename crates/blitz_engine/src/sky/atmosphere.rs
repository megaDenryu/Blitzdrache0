//! 大気媒体方針: その世界の大気が何でできているかを、時刻に依存しない形で表した方針の語彙。
//! プリコンピュートのベイク済み画像(透過率・多重散乱・スカイビュー・空中遠近)の生成入力であり、
//! 時刻から導く天空状態とは別の軸で持つ。ここにあるのは値と不変条件だけであり、ベイク済み画像を作る数学は
//! `blitz_render`の大気数学が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」

mod absorption_component;
mod albedo_rgb;
mod asymmetry;
mod density_layer;
mod density_profile;
mod extinction_rgb;
mod key_digest;
mod medium_policy;
mod mie_component;
mod planet_shape;
mod rayleigh_component;
mod static_key;

#[cfg(test)]
mod atmosphere_tests;

pub use absorption_component::吸収成分;
pub use albedo_rgb::地表アルベドRGB;
pub use asymmetry::位相非対称係数;
pub use density_layer::密度分布層;
pub use density_profile::密度分布;
pub use extinction_rgb::消散係数RGB;
pub use medium_policy::大気媒体方針;
pub use mie_component::ミー成分;
pub use planet_shape::惑星形状;
pub use rayleigh_component::レイリー成分;
pub use static_key::大気静的キー;
