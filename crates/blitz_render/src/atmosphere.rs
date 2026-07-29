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

mod asymmetry;
mod attenuating_component;
mod density;
mod density_layer;
mod density_profile;
mod error;
mod extinction_rgb;
mod inner_radius;
mod intersect;
mod medium;
mod narrowing;
mod normalized_density;
mod optical_length;
mod phase;
mod phase_value;
mod sample_count;
mod scattering_cosine;
mod transmittance;
mod transmittance_rgb;
mod zenith_cosine;

#[cfg(test)]
mod atmosphere_tests;

pub use asymmetry::位相非対称係数;
pub use attenuating_component::減衰成分;
pub use density::規格化密度を求める;
pub use density_layer::密度分布層;
pub use density_profile::密度分布;
pub use error::大気数学エラー;
pub use extinction_rgb::消散係数RGB;
pub use inner_radius::大気内半径;
pub use intersect::{地平線の天頂余弦, 大気上端までの距離, 惑星までの距離, 視線が惑星と交差するか};
pub use medium::消散媒体;
pub use normalized_density::規格化密度;
pub use optical_length::上端までの光学距離;
pub use phase::{ミー位相関数, レイリー位相関数};
pub use phase_value::位相関数値;
pub use sample_count::積分標本数;
pub use scattering_cosine::散乱角余弦;
pub use transmittance::上端までの透過率;
pub use transmittance_rgb::透過率RGB;
pub use zenith_cosine::天頂余弦;
