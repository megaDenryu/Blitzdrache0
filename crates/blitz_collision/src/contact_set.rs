//! 接触点集合: 重なっている(または接している)2つの形が触れ合う点をまとめて求める。剛体が箱を積んで安定させ、
//! 摩擦を計算するための前提である。
//!
//! 担当するのは、直方体どうしと、直方体と三角形の2つの組である。方式はどちらも同じ2段であり、分離軸で重なりと
//! 最小の貫通の軸を求め、参照面と入射面の切り抜きで最大数点の接触点を作る。最小の軸が2つの辺の向きの外積で
//! あるときは、2本の辺の最近接の1点だけになる。
//!
//! 1点の接触を返す掃引(`crate::oriented_box`・`crate::triangle`・`crate::capsule`)と別のモジュールにするのは、
//! 答える問いが違うためである。掃引が答えるのは「動かしたときいつ触れ始めるか」であり、こちらは「今どこで
//! どれだけ重なっているか」である。前者は時刻を1つ返し、後者は同じ1つの時刻の断面で複数の点を返す。
//!
//! この段で作らないものが3つある。1つはカプセルの接触点集合であり、キャラクターの移動は掃引の1点接触で
//! 足りるため、剛体がカプセルを積む要求が立った日に足す。もう1つは接触点の時間方向の維持と暖機(前の刻みの
//! 力を引き継いで収束を速めること)であり、拘束の解法(#8)の担当である。この層は追跡の綴りになる特徴の識別を
//! 返すところまでを持つ。3つ目は世界側への配線であり、利用者は剛体(#7)である。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断13: 連続衝突と接触点集合を能力として最初から見据える」
//!
//! 決定性。分離軸の並びも、面の輪郭の頂点の並びも、側の平面の並びもすべて固定であり、同じ深さの軸が並んだ
//! ときは先に来たものが残る。刻み探索も反復法も使わないため、同じ入力は必ずビット単位で同じ集合を同じ順で
//! 返す(`_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」)。
//!
//! 確保。1回の問い合わせでヒープを確保しない。切り抜きの途中の多角形も接触点の受け皿も上限つきの固定の長さの
//! 配列であり、上限の根拠は `crates/blitz_collision/src/contact_set/manifold_capacity.rs` にある。
//!
//! 精度。判定はすべて倍精度で行う。単精度へ狭めるのは共有の単位法線を返す1箇所だけであり、接触点の位置も
//! 符号付き貫通量も倍精度のまま返る。
//!
//! 余白。2つの問い合わせは接触生成の余白を受け取り、参照面の平面よりその長さまで外側に居る切り抜きの角を、負の
//! 符号付き貫通量を持つ接触点として残す。余白を持たない問い合わせは食い込んでいる角だけを残す。理由と、余白を
//! 外した反証が塔の場面で何を落とすかは `crates/blitz_collision/src/contact_set/generation_margin.rs` にある。

mod axis_interval;
mod box_clip_parts;
mod box_feature_support;
mod box_pair_axis;
mod box_pair_axis_search;
mod box_pair_feature;
mod box_pair_manifold;
mod box_pair_query;
mod box_projection;
mod box_triangle_axis;
mod box_triangle_axis_search;
mod box_triangle_edge_contact;
mod box_triangle_face_manifold;
mod box_triangle_feature;
mod box_triangle_manifold;
#[cfg(test)]
mod box_triangle_margin_tests;
mod box_triangle_query;
mod clip_feature_pair;
mod clip_plane;
mod clip_polygon;
mod clip_reference_feature;
mod cross_axis_margin;
mod deepest_axis;
mod edge_pair_contact;
mod error;
mod generation_margin;
mod manifold;
mod manifold_builder;
mod manifold_capacity;
mod penetration_depth;
mod reference_frame;
mod reference_role;
mod segment_pair_point;
mod solver_candidate_point;
mod triangle_clip_parts;
mod triangle_projection;

#[cfg(test)]
mod box_pair_answer_fixture;
#[cfg(test)]
mod box_pair_completeness_tests;
#[cfg(test)]
mod box_pair_edge_tests;
#[cfg(test)]
mod box_pair_face_tests;
#[cfg(test)]
mod box_pair_invariant_tests;
#[cfg(test)]
mod box_pair_stack_tests;
#[cfg(test)]
mod box_triangle_answer_fixture;
#[cfg(test)]
mod box_triangle_tests;
#[cfg(test)]
mod determinism_tests;
#[cfg(test)]
mod feature_order_tests;
#[cfg(test)]
mod penetration_depth_tests;
#[cfg(test)]
mod query_fixture;
#[cfg(test)]
mod random_box_fixture;

pub use box_pair_feature::直方体どうしの接触の特徴の対;
pub use box_pair_query::{二つの直方体の重なりの接触点集合, 直方体どうしの接触点集合の問い合わせ};
pub use box_triangle_feature::直方体と三角形の接触の特徴の対;
pub use box_triangle_query::{直方体と三角形の接触点集合の問い合わせ, 直方体と三角形の重なりの接触点集合};
pub use error::接触点集合の問い合わせエラー;
pub use generation_margin::接触生成の余白メートル;
pub use manifold::接触点集合;
pub use manifold_capacity::接触点の上限;
pub use penetration_depth::符号付き貫通量メートル;
pub use solver_candidate_point::接触解法の候補点;
