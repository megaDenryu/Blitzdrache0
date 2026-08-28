//! 任意姿勢の直方体への詳細判定。担当するのは、線分が直方体に最初に当たる点を求める問い合わせと、
//! カプセルを掃引して直方体に最初に触れるまでを求める問い合わせである。
//!
//! 形そのもの(`crate::shape`の任意姿勢の直方体)と別のモジュールにするのは、形の表現が焼き込みにも空間索引にも
//! 現れるのに対し、詳細判定が実行中の問い合わせにしか現れないためである。判定の側から形の側への依存だけがある。
//!
//! 数学の方式は2つである。線分は、直方体の3本の軸へ射影して軸ごとに線分が板の中に居る時刻の区間を作り、
//! 3つの区間を交差させる。カプセルの掃引は、動く軸の線分と直方体の特徴(面6・辺12・頂点8)との隔たりが
//! 半径に等しくなる最小の時刻を、特徴ごとの2次方程式として解析的に解く。どちらも刻み探索も二分法も反復法も
//! 使わない。同じ入力が必ずビット単位で同じ答えを返すためである
//! (`_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」)。
//!
//! 座標系と丸め。問い合わせの点と向きは、直方体の中心と同じ1つの局所座標系に倍精度で来る。この局所座標系まで
//! 写すのは呼び出し側(世界の側)であり、大域の倍精度の位置から局所の原点を引く減算の丸めがそこで1回入る。
//! 直方体自身は単精度で焼かれているため、中心・回転・半分の長さを倍精度へ広げる読み直しに丸めは入らない
//! (四元数の正規化だけが倍精度の割り算を1回通る)。判定の演算はすべて倍精度で行い、単精度へ狭めるのは
//! 接触の単位法線を`blitz_math`の方向として返す1箇所だけである。
//!
//! 桁あふれ。問い合わせの点と向きの成分の絶対値は、形の座標と同じ上限(2の24乗メートル)に収まる。この上限をBと
//! 書くと、直方体の角と面の代表点は成分の絶対値が4B以下、そこから軸の端へ向かう変位は6B以下である。掃引の
//! 候補のうち係数が最も大きく育つのは辺と軸の内側の式であり、高さ場の三角形と同じ数え方で判別式は
//! 11059200かける6Bの12乗以下になる。6の12乗が2176782336であるから、これは24073471210291200かけるBの12乗
//! である。B = 2の24乗ではBの12乗が2の288乗すなわちおよそ5.0かける10の86乗であるから、判別式の上界はおよそ
//! 1.2かける10の103乗であり、倍精度の最大値1.8かける10の308乗を大きく下回る。
//! 数え方の元は `crates/blitz_collision/src/height_field/length_limit.rs` にある。

mod capsule;
mod error;
mod feature;
mod feature_enumeration;
mod feature_geometry;
#[cfg(test)]
mod feature_geometry_tests;
mod feature_normal;
mod feature_place;
#[cfg(test)]
mod query_fixture;
#[cfg(test)]
mod segment_axis_aligned_tests;
mod segment_entry;
mod segment_hit;
mod segment_query;
#[cfg(test)]
mod segment_rotated_tests;
mod segment_span;
#[cfg(test)]
mod segment_tangent_tests;
mod segment_through_box;
mod slab;
mod start_inside;
mod sweep_candidate;
mod sweep_contact;
mod sweep_dispatch;
#[cfg(test)]
mod sweep_feature_tests;
#[cfg(test)]
mod sweep_fixture;
mod sweep_geometry;
mod sweep_place;
mod sweep_query;
mod sweep_solve_edge;
mod sweep_solve_edge_axis;
mod sweep_solve_face;
mod sweep_solve_vertex;
mod sweep_solver;
#[cfg(test)]
mod sweep_start_tests;

pub use capsule::直方体を掃引するカプセル;
pub use error::直方体の問い合わせエラー;
pub use feature::{直方体で当たった特徴, 直方体の辺, 直方体の面, 直方体の頂点};
pub use segment_hit::{直方体への最初の当たり, 線分が直方体に最初に当たる点};
pub use segment_query::直方体の線分の問い合わせ;
pub use segment_through_box::直方体を貫く線分;
pub use sweep_contact::{カプセルが直方体に最初に触れる点, 掃引したカプセルの直方体への接触};
pub use sweep_query::直方体のカプセルの掃引の問い合わせ;
