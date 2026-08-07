//! 時間再構成: 過去のフレームの結果を今のフレームへ混ぜ合わせる方式の純関数の層。
//! 動きベクトルの規約・履歴の混合・棄却・画素内ずらしの列と語彙の数が多いため、クレートの平坦な再エクスポートへ
//! 混ぜずモジュールごと公開する。
//!
//! この層はGPUもフレームも実時間も知らず、同じ入力からは常に同じ出力を返す。自動露出と局所可視性補正で
//! 確立した様式をそのまま使う。フレームの経路への配線・資源・シェーダーはこの層の外にあり、ここで決めた
//! 規約と関数を段3a以降が使う。
//! 参照: `_doc/設計/時間再構成.md`「判断f: 純関数層の中身」
//!
//! この層が最初に固定するのは動きベクトルの規約である。向き・単位・画素内ずらしの差分の含否の3つは、
//! 書く側と読む側の全てへ波及するため、他のどの実装よりも先に型で決める(`motion_vector.rs`が根拠を持つ)。
//!
//! 段3bで入るslangの写しと同じ値でなければならない定数は`constants.rs`が1箇所で持つ。
//! `cargo xtask conform`の定数一致検査への登録は、写しが現れる段3bで行う。

mod constants;
mod current_frame_ratio;
mod disocclusion_tolerance;
mod error;
mod exponential_moving_average;
mod history_admission;
mod jitter_sequence;
mod linear_color;
mod luminance_weighted_blend;
mod motion_vector;
mod neighborhood_range;
mod pixel_offset;
mod relative_luminance;
mod screen_position;
mod sequence_index;
mod view_depth;

#[cfg(test)]
mod admission_tests;
#[cfg(test)]
mod blend_tests;
#[cfg(test)]
mod clamp_tests;
#[cfg(test)]
mod jitter_sequence_tests;
#[cfg(test)]
mod luminance_weight_tests;
#[cfg(test)]
mod motion_vector_tests;
#[cfg(test)]
mod value_object_tests;

pub use current_frame_ratio::今のフレームの寄与率;
pub use disocclusion_tolerance::出現領域の相対許容;
pub use error::時間再構成エラー;
pub use exponential_moving_average::指数移動平均で混ぜる;
pub use history_admission::{履歴の採否, 履歴を採るか判定する};
pub use jitter_sequence::画素を単位とするずらしを求める;
pub use linear_color::線形色;
pub use luminance_weighted_blend::輝度で重み付けて混ぜる;
pub use motion_vector::動きベクトル;
pub use neighborhood_range::近傍の色の範囲;
pub use pixel_offset::画素を単位とするずらし;
pub use relative_luminance::{相対輝度, 相対輝度を求める};
pub use screen_position::画面上の位置;
pub use sequence_index::ずらしの列の添字;
pub use view_depth::ビュー空間の奥行き;
