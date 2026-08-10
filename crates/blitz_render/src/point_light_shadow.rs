//! 点光源の影の数学。影を持てる灯の件数と層の割り当て、面ごとの投影、比較深度の作り方を持つ。
//! ここにあるのは純粋な計算だけであり、Vulkanの資源も描画の順序も知らない。
//! ashに触れないため、平坦な再エクスポートへ混ぜずモジュールごと公開し、型を`point_light_shadow::`で辿れるようにする。
//!
//! このモジュールが投影の契約の正本であり、`shaders/point_light_shadow_projection.slang`がその写しになる
//! (多段シャドウの距離区分のブレンドで`cascade/blend.rs`が正本、`shaders/cascade_shadow.slang`が写しであるのと同じ関係)。
//! CPU側に正本を置くのは、近面と遠面での値・主軸が同値になる境界での一致・面ごとの投影と立方体の写像の一致を
//! ユニットテストで検査できるようにするためである。
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断l」「判断m」

mod capacity;
mod depth_bias;
mod face_basis;
mod face_frustum;
mod input_error;
mod major_axis_distance;
mod projection_contract;
mod shadow_resource_index;

#[cfg(test)]
mod comparison_depth_tests;
#[cfg(test)]
mod face_frustum_tests;
#[cfg(test)]
mod projection_contract_tests;
#[cfg(test)]
mod shadow_resource_index_tests;
#[cfg(test)]
mod test_fixture;

pub use capacity::{影を持てる灯の上限件数, 点光源の影の層の総数, 点光源の影の面の一辺};
pub use depth_bias::点光源の影の深度の偏りのメートル;
pub use face_basis::{面の直交基底, 面の直交基底を求める};
pub use face_frustum::面の錐台;
pub use input_error::点光源の影の入力エラー;
pub use major_axis_distance::光から表面へのベクトルの主軸の距離を求める;
pub use projection_contract::投影の契約;
pub use shadow_resource_index::{層番号から影資源添字と面を求める, 影資源添字};
