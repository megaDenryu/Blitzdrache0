//! カプセルの掃引と高さ場の接触。答えるのは「カプセルを指定した変位だけ動かそうとしたとき、
//! 高さ場の地表に触れるまでにどこまで動けるか」であり、返すのは動けた割合・接触位置・接触の単位法線・
//! どの升目のどちらの三角形のどの特徴に触れたかである。
//!
//! 数学の方式は、動くカプセルの軸の線分と三角形の特徴(面・3辺・3頂点)との隔たりが半径に等しくなる
//! 最小の時刻を、特徴ごとの2次方程式(面と軸の端点の組は1次)として解析的に解くものである。
//! 数値の刻み探索も二分法も反復法も使わない。同じ入力が必ずビット単位で同じ答えを返すためである
//! (`_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」)。
//!
//! 調べる升目を選ぶ方式は線分の問い合わせと違う。線分は横断した升目だけを時刻の順に辿れるが、掃引の覆いは
//! 線ではなく面であり、面の中の升目を接触の時刻の順に並べる手立てが無いためである。覆いを包む軸平行の矩形の
//! 升目をすべて調べ、最小の時刻を選ぶ。隣り合う升目が共有する辺と頂点は複数回調べられるが、
//! 同じ時刻を複数回求めるだけであり最小の選択で畳まれる。

mod axis_end;
mod candidate;
mod candidate_dispatch;
mod capsule;
mod contact;
mod contact_candidate;
mod contact_place;
mod contact_solver;
mod covering;
mod crossing;
mod error;
mod feature;
mod line_pair;
mod quadratic;
mod ratio;
mod scanned_range;
mod smallest;
mod solve_edge;
mod solve_edge_axis;
mod solve_face;
mod solve_vertex;
mod solver_geometry;
mod surface_triangle;
mod surface_triangle_inside;
#[cfg(test)]
mod sweep_end_tests;
#[cfg(test)]
mod sweep_feature_tests;
mod sweep_hit;
mod sweep_query;
mod sweep_query_cell;
#[cfg(test)]
mod sweep_query_fixture;
#[cfg(test)]
mod sweep_query_tests;
mod sweep_result;
#[cfg(test)]
mod sweep_start_tests;
mod triangle_answer;

pub use capsule::高さ場を掃引するカプセル;
pub use contact::掃引したカプセルの地表への接触;
pub use error::カプセルの掃引の問い合わせエラー;
pub use feature::地表に触れた特徴;
pub use ratio::掃引で動けた割合;
pub use scanned_range::掃引の走査が調べた範囲;
pub use sweep_hit::カプセルが地表に最初に触れる点;
pub use sweep_query::高さ場のカプセルの掃引の問い合わせ;
pub use sweep_result::カプセルの掃引と高さ場の接触の結果;
