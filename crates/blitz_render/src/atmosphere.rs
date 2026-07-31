//! 大気の数学。高度から密度、視線と球の交差、経路に沿った光学距離、透過率、散乱の角度分布を求める。
//! ここにあるのは純粋な計算だけであり、Vulkanの資源も描画の順序も知らない。
//! ashに触れないため、平坦な再エクスポートへ混ぜずモジュールごと公開し、型を`atmosphere::`で辿れるようにする。
//!
//! このモジュールがプリコンピュートのベイク済み画像の正本であり、コンピュートシェーダーはこの式の写しになる
//! (カスケードシャドウの距離区分のブレンドで`cascade/blend.rs`が正本、`shaders/cascade_shadow.slang`が写しであるのと同じ関係)。
//! CPU側に正本を置くのは、閉形式との一致・正規化・単調性といった性質をユニットテストで検査できるようにするためである。
//!
//! 内側は関心事ごとに7つの子モジュールへ分かれる。量(`quantity`)・媒体(`medium`)・幾何(`geometry`)・
//! 写像(`mapping`)・表(`table`)・積分(`integration`)・焼き上げ(`bake`)であり、依存はこの並びの向きにだけ流れる。
//! この階層を採るのは、ベイク済み画像が1枚増えるたびに写像・積分・焼き上げの3つがそろって増え、平坦な並びでは
//! どの3つが1組なのかも、どの関心事に属するのかも読めなくなるためである。外から見える名前はこのファイルの
//! 再エクスポートが決めるため、階層の変更が呼び出し側へ漏れない。
//!
//! 計算はf64で行い、f32へ狭めるのは呼び出し側へ返す境界だけである。理由は`narrowing.rs`にある。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」

mod bake;
mod error;
mod geometry;
mod integration;
mod mapping;
mod medium;
mod narrowing;
mod quantity;
mod table;

#[cfg(test)]
mod atmosphere_tests;

pub use bake::aerial_condition::空中遠近観測条件;
pub use bake::aerial_lut::{
    空中遠近ボリュームのボクセル値, 空中遠近ボリュームの材料, 空中遠近ボリュームを焼く
};
pub use bake::multiscatter_lut::{多重散乱のベイク済み画像のテクセル値, 多重散乱のベイク済み画像を焼く};
pub use bake::skyview_condition::スカイビュー観測条件;
pub use bake::skyview_lut::{スカイビューのベイク済み画像のテクセル値, スカイビューのベイク済み画像を焼く};
pub use bake::transmittance_lut::{透過率のベイク済み画像のテクセル値, 透過率のベイク済み画像を焼く};
pub use error::大気数学エラー;
pub use geometry::frustum_rays::視錐台の向き;
pub use geometry::intersect::{地平線の天頂余弦, 大気上端までの距離, 惑星までの距離, 視線が惑星と交差するか};
pub use integration::optical_length::上端までの光学距離;
pub use integration::phase::{ミー位相関数, レイリー位相関数};
pub use integration::sun_visibility;
pub use integration::transmittance::上端までの透過率;
pub use mapping::aerial_mapping::{
    空中遠近ボリュームの奥行き位置を求める, 空中遠近ボリュームの奥行き添字の位置, 空中遠近ボリュームの標本座標を求める,
    空中遠近ボリュームの画面位置を求める, 空中遠近ボリュームの距離を求める,
};
pub use mapping::lut_resolution::大気のベイク済み画像の解像度;
pub use mapping::multiscatter_mapping::{
    多重散乱のベイク済み画像の単位位置を求める, 多重散乱のベイク済み画像の条件, 多重散乱のベイク済み画像の条件を求める,
};
pub use mapping::skyview_lookup::ワールドの視線からスカイビューのベイク済み画像の視線を求める;
pub use mapping::skyview_mapping::{
    スカイビューのベイク済み画像の単位位置を求める, スカイビューのベイク済み画像の視線, スカイビューのベイク済み画像の視線を求める,
};
pub use mapping::transmittance_mapping::{
    テクセル中心のuv, 透過率のベイク済み画像のuvを求める, 透過率のベイク済み画像の視線, 透過率のベイク済み画像の視線を求める,
};
pub use mapping::unit_texel::{単位位置からテクセル中心のuv, 端から端への単位位置};
pub use medium::density::規格化密度を求める;
pub use medium::density_layer::密度分布層;
pub use medium::density_profile::密度分布;
pub use medium::extinction_medium::消散媒体;
pub use medium::observation_point::大気内観測点;
pub use medium::scattering_medium::大気散乱媒体;
pub use quantity::aerial_voxel::空中遠近ボクセル値;
pub use quantity::albedo_rgb::地表反射率RGB;
pub use quantity::asymmetry::位相非対称係数;
pub use quantity::attenuating_component::減衰成分;
pub use quantity::azimuth_cosine::方位余弦;
pub use quantity::extinction_rgb::消散係数RGB;
pub use quantity::multiscatter_rgb::多重散乱RGB;
pub use quantity::normalized_density::規格化密度;
pub use quantity::phase_value::位相関数値;
pub use quantity::sample_count::積分標本数;
pub use quantity::scattering_cosine::散乱角余弦;
pub use quantity::skyview_rgb::スカイビュー放射輝度RGB;
pub use quantity::transmittance_rgb::透過率RGB;
pub use quantity::unit_direction::単位方向;
pub use quantity::zenith_cosine::天頂余弦;
pub use table::multiscatter_table::多重散乱表;
pub use table::skyview_table::スカイビュー表;
pub use table::transmittance_table::透過率表;
