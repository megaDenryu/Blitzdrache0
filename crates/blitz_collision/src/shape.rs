//! 形状の表現: 物理の問い合わせが相手にする形そのものと、その形を粗く包む境界を持つ。
//!
//! ここに在るのは形だけであり、その形がどの物体に属するか・どのチャンクで読み込まれたか・いつ消えるかは
//! 世界側の関心である。形の表現を衝突数学層が一意に所有するのは、同じ形へ複数の層が別々の表現を持つと、
//! 判定に使う形と焼き込みや描画に渡す形がいつか離れるためである。
//!
//! 実装順5の時点で持つのは、任意姿勢の直方体と、それを包む軸平行の直方体の2つである。判断5が挙げた残りの形状
//! (球・カプセル・円柱・凸多面体・三角形網)は実装順6以降でここへ足す。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「層の定義」「判断5: 形状の目録を先に確定する(物理形状と描画形状は別物)」

mod axis_aligned_box;
#[cfg(test)]
mod axis_aligned_box_tests;
mod coordinate_limit;
mod error;
mod global_axis_aligned_box;
mod half_extent;
mod oriented_box;
mod oriented_box_bounds;
#[cfg(test)]
mod oriented_box_bounds_tests;

pub use axis_aligned_box::軸平行の直方体;
pub use coordinate_limit::形状の座標の絶対値の上限メートル;
pub use error::直方体の生成エラー;
pub use global_axis_aligned_box::大域の軸平行の直方体;
pub use half_extent::直方体の軸ごとの半分の長さ;
pub use oriented_box::任意姿勢の直方体;
