//! 一般の三角形への詳細判定。担当するのは、線分が三角形に最初に当たる点を求める問い合わせと、カプセルを
//! 掃引して三角形に最初に触れるまでを求める問い合わせである。
//!
//! 形そのもの(`crate::shape`の局所座標の三角形)と別のモジュールにするのは、形の表現が三角形網の保持にも空間
//! 索引にも現れるのに対し、詳細判定が実行中の問い合わせにしか現れないためである。判定の側から形の側への依存
//! だけがある。
//!
//! 高さ場の三角形への判定(`crate::height_field`)と別に置くのは、あちらが升目の四隅の高さから面を組み立て、
//! 位置と変位を格子原点を基準にした型で持つためである。こちらは頂点の位置しか持たず、形の局所座標系に住む。
//! 求解の式そのものは`crate::solver`が両方へ与えるため、書き写してはいない。
//!
//! 数学の方式は2つである。線分は、3頂点が張る平面を横切る時刻を1次式で解き、その点が三角形の内側に在るかを
//! 辺ごとの符号で見る。カプセルの掃引は、動く軸の線分と三角形の特徴(面1・辺3・頂点3)との隔たりが半径に
//! 等しくなる最小の時刻を、特徴ごとの2次方程式として解析的に解く。どちらも刻み探索も二分法も反復法も使わない。
//! 同じ入力が必ずビット単位で同じ答えを返すためである
//! (`_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」)。
//!
//! 表裏。当面はどちらの面からも当たる。表側だけ衝突するかどうかは衝突形状が宣言する属性であり、この層の
//! 幾何が決めるものではない(`_doc/設計/世界の形と衝突基盤.md`「判断10: 衝突形状が宣言できる属性」)。
//! 属性を足すまでの間、線分の当たりが返す単位法線は線分の始点が在る側を向く。
//!
//! 桁あふれ。三角形の頂点・カプセルの軸の端・掃引の変位・半径は、どれも形の座標と同じ上限(2の24乗メートル)に
//! 収まる。求解が導く点は係数にも判別式にも入らず、係数を組むのに使うのは時刻0の値だけである。したがって
//! 数え方は高さ場の三角形とまったく同じであり、上限をBと書くと判別式は11059200かけるBの12乗以下になる。
//! B = 2の24乗ではおよそ5.5かける10の93乗であり、倍精度の最大値1.8かける10の308乗を大きく下回る。任意姿勢の
//! 直方体がこれより大きい上界を持つのは、あちらの角と面の代表点が中心と半分の長さから導いた点であり、成分が
//! 上限の4倍まで出るためである。数え方の元は `crates/blitz_collision/src/height_field/length_limit.rs` にある。

mod capsule;
mod error;
mod feature;
#[cfg(test)]
mod query_fixture;
mod segment;
mod segment_feature;
#[cfg(test)]
mod segment_feature_tests;
mod segment_hit;
mod segment_query;
#[cfg(test)]
mod segment_tests;
mod sweep_candidate;
mod sweep_contact;
mod sweep_dispatch;
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
mod sweep_start_overlap;
#[cfg(test)]
mod sweep_tests;

pub use capsule::三角形を掃引するカプセル;
pub use error::三角形の問い合わせエラー;
pub use feature::三角形で当たった特徴;
pub use segment::三角形を貫く線分;
pub use segment_hit::{三角形への最初の当たり, 線分が三角形に最初に当たる点};
pub use segment_query::三角形の線分の問い合わせ;
pub use sweep_contact::{カプセルが三角形に最初に触れる点, 掃引したカプセルの三角形への接触};
pub use sweep_query::三角形のカプセルの掃引の問い合わせ;
