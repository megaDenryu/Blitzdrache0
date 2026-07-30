//! 大気の数学。高度から密度、視線と球の交差、経路に沿った光学距離、透過率、散乱の角度分布を求める。
//! ここにあるのは純粋な計算だけであり、Vulkanの資源も描画の順序も知らない。
//! ashに触れないため、平坦な再エクスポートへ混ぜずモジュールごと公開し、型を`atmosphere::`で辿れるようにする。
//!
//! このモジュールがプリコンピュートLUTの正本であり、後段で入れるコンピュートシェーダーはこの式の写しになる
//! (カスケードシャドウの帯ブレンドで`cascade/blend.rs`が正本、`shaders/cascade_shadow.slang`が写しであるのと同じ関係)。
//! CPU側に正本を置くのは、閉形式との一致・正規化・単調性といった性質をユニットテストで検査できるようにするためである。
//!
//! 計算はf64で行い、f32へ狭めるのは呼び出し側へ返す境界だけである。理由は`narrowing.rs`にある。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」

mod albedo_rgb;
mod asymmetry;
mod attenuating_component;
mod azimuth_cosine;
mod bilinear;
mod density;
mod density_layer;
mod density_profile;
mod error;
mod extinction_rgb;
mod intersect;
mod lut_resolution;
mod medium;
mod medium_sample;
mod multiscatter_lut;
mod multiscatter_mapping;
mod multiscatter_march;
mod multiscatter_rgb;
mod multiscatter_series;
mod multiscatter_step;
mod multiscatter_table;
mod narrowing;
mod normalized_density;
mod observation_point;
mod optical_length;
mod phase;
mod phase_value;
mod sample_count;
mod scattering_cosine;
mod scattering_medium;
mod skyview_condition;
mod skyview_lookup;
mod skyview_lut;
mod skyview_mapping;
mod skyview_march;
mod skyview_rgb;
mod skyview_step;
mod sphere_directions;
mod sun_visibility;
mod transmittance;
mod transmittance_lut;
mod transmittance_mapping;
mod transmittance_rgb;
mod transmittance_table;
mod unit_texel;
mod vector3;
mod zenith_cosine;

#[cfg(test)]
mod atmosphere_tests;

pub use albedo_rgb::地表アルベドRGB;
pub use asymmetry::位相非対称係数;
pub use attenuating_component::減衰成分;
pub use azimuth_cosine::方位余弦;
pub use density::規格化密度を求める;
pub use density_layer::密度分布層;
pub use density_profile::密度分布;
pub use error::大気数学エラー;
pub use extinction_rgb::消散係数RGB;
pub use intersect::{地平線の天頂余弦, 大気上端までの距離, 惑星までの距離, 視線が惑星と交差するか};
pub use lut_resolution::大気LUT解像度;
pub use medium::消散媒体;
pub use multiscatter_lut::{多重散乱lutのテクセル値, 多重散乱lutを焼く};
pub use multiscatter_mapping::{多重散乱lutの単位位置を求める, 多重散乱lutの条件, 多重散乱lutの条件を求める};
pub use multiscatter_rgb::多重散乱RGB;
pub use multiscatter_table::多重散乱表;
pub use normalized_density::規格化密度;
pub use observation_point::大気内観測点;
pub use optical_length::上端までの光学距離;
pub use phase::{ミー位相関数, レイリー位相関数};
pub use phase_value::位相関数値;
pub use sample_count::積分標本数;
pub use scattering_cosine::散乱角余弦;
pub use scattering_medium::大気散乱媒体;
pub use skyview_condition::スカイビュー観測条件;
pub use skyview_lookup::ワールドの視線からスカイビューlutの視線を求める;
pub use skyview_lut::{スカイビューlutのテクセル値, スカイビューlutを焼く};
pub use skyview_mapping::{
    スカイビューLUT視線, スカイビューlutの単位位置を求める, スカイビューlutの視線を求める
};
pub use skyview_rgb::スカイビュー放射輝度RGB;
pub use transmittance::上端までの透過率;
pub use transmittance_lut::{透過率lutのテクセル値, 透過率lutを焼く};
pub use transmittance_mapping::{テクセル中心のuv, 透過率LUT視線, 透過率lutのuvを求める, 透過率lutの視線を求める};
pub use transmittance_rgb::透過率RGB;
pub use transmittance_table::透過率表;
pub use unit_texel::{単位位置からテクセル中心のuv, 端から端への単位位置};
pub use zenith_cosine::天頂余弦;
