//! 形の表現: 物理の問い合わせが相手にする形そのものと、その形を粗く包む境界と、形が住む局所座標系の位置と変位を持つ。
//!
//! ここに在るのは形だけであり、その形がどの物体に属するか・どのチャンクで読み込まれたか・いつ消えるかは
//! 世界側の関心である。形の表現を衝突数学層が一意に所有するのは、同じ形へ複数の層が別々の表現を持つと、
//! 判定に使う形と焼き込みや描画に渡す形がいつか離れるためである。
//!
//! 位置と変位を形と同じところへ置くのは、形の中心と問い合わせの点が同じ1つの局所座標系に住むためである。
//! 別のところへ置くと、形の側と問い合わせの側で座標系の定義が2つに割れる。
//!
//! 実装順7の時点で持つのは、任意姿勢の直方体と、それを包む軸平行の直方体と、直方体を倍精度で読み直した幾何と、
//! 3つの頂点で表す三角形と、形の内側の空間索引が使う倍精度の軸平行の直方体である。判断5が挙げた残りの形状
//! (球・円柱・凸多面体)は実装順8以降でここへ足す。
//!
//! 問い合わせの入力になる形(形を貫く線分・カプセル・形を掃引するカプセル)もここに在る。これらは相手の形の
//! 種類に依らず同じ局所座標系の同じ不変条件を持つため、三角形用と直方体用に写した2つの型を1つへ畳んだもので
//! ある。高さ場への入力は格子原点を基準にした別の座標系に住むため、この統合の対象ではない。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「層の定義」「判断5: 形状の目録を先に確定する(物理形状と描画形状は別物)」

mod axis_aligned_box;
#[cfg(test)]
mod axis_aligned_box_tests;
mod box_axis;
mod box_face_side;
mod capsule;
mod capsule_error;
mod capsule_radius_lower_limit;
#[cfg(test)]
mod capsule_radius_lower_limit_tests;
mod coordinate_limit;
mod cover_rounding_margin;
mod error;
mod global_axis_aligned_box;
mod global_box_components;
mod global_box_cover;
#[cfg(test)]
mod global_box_rounding_margin_tests;
mod half_extent;
mod local_axis;
mod local_axis_aligned_box;
#[cfg(test)]
mod local_axis_aligned_box_tests;
mod local_box_cover;
mod local_displacement;
mod local_displacement_direction;
mod local_displacement_solving;
mod local_error;
mod local_position;
mod moving_oriented_box;
mod oriented_box;
mod oriented_box_bounds;
#[cfg(test)]
mod oriented_box_bounds_fixture;
#[cfg(test)]
mod oriented_box_bounds_margin_tests;
#[cfg(test)]
mod oriented_box_bounds_tests;
mod oriented_box_double;
mod outward_narrowing;
#[cfg(test)]
mod outward_narrowing_tests;
mod shape_segment;
mod sweep_capsule;
mod triangle;
mod triangle_face_orientation;
mod triangle_inside;
#[cfg(test)]
mod triangle_tests;
mod triangle_vertex_number;
mod triangle_with_orientation;

pub use axis_aligned_box::軸平行の直方体;
pub use box_axis::直方体自身の座標軸;
pub use box_face_side::直方体の面の向き;
pub use capsule::カプセル;
pub use capsule_error::カプセルの生成エラー;
pub use capsule_radius_lower_limit::カプセルの半径の下限メートル;
pub use coordinate_limit::形状の座標の絶対値の上限メートル;
pub use error::直方体の生成エラー;
pub use global_axis_aligned_box::大域の軸平行の直方体;
pub use half_extent::直方体の軸ごとの半分の長さ;
pub use local_axis_aligned_box::局所座標の軸平行の直方体;
pub use local_displacement::形の局所座標の変位;
pub use local_error::形の局所座標の生成エラー;
pub use local_position::形の局所座標の位置;
pub use moving_oriented_box::動く任意姿勢の直方体;
pub use oriented_box::任意姿勢の直方体;
pub use oriented_box_double::任意姿勢の直方体の倍精度の幾何;
pub use shape_segment::形を貫く線分;
pub use sweep_capsule::形を掃引するカプセル;
pub use triangle::局所座標の三角形;
pub use triangle_face_orientation::三角形の面の向き;
pub use triangle_vertex_number::三角形の頂点の番号;
pub use triangle_with_orientation::面の向きを備えた三角形;

pub(crate) use local_axis::形の局所座標の軸;
