//! 接触点集合1つの法線と接線を同じ対称行列へ入れて同時に解く連立(判断12・判断13)。
//! 非貫通(判断12)を解き終えた配置から始め、法線の行は隔たりを動かさないこと、接線の行は錨の接線変位を零へ
//! 戻すことを同時に求める。求まるのは、接線の補正と、それが作る回転を打ち消す法線の荷重の移し替えである。
//! 接線だけを解くと、接触点に働く接線の力が重心まわりの回転を含む補正を作り、次の反復の非貫通がその回転を
//! 戻して錨の接線変位を作り直す。接線の乗数は作り直された変位に対してもう一度積まれ、傾きの正接が静止摩擦
//! 係数より小さい坂でも円錐を超える(実測は `_doc/計測/剛体の接触の摩擦と反発_2026-09-03.md`)。
//! 行が2つの参加者への符号付きの勾配を持つのは、法線と接線の交差の項の符号が参加者ごとの勾配で決まるためである。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

mod active_set;
mod normal_row;
#[cfg(test)]
mod order_tests;
mod reduced_system;
mod row_gradient;
mod row_order;
#[cfg(test)]
mod single_point_fixture;
#[cfg(test)]
mod single_point_tests;
mod solution;
mod solve_outcome;
mod system;
#[cfg(test)]
mod system_fixture;
mod system_solve;
mod tangential_row;
#[cfg(test)]
mod tolerance_tests;
#[cfg(test)]
mod translation_tests;
mod visited_active_set;
#[cfg(test)]
mod visited_active_set_tests;

pub use normal_row::法線の隔たりを動かさない一行;
pub use system::接触点集合の法線と接線の連立;
pub use tangential_row::錨の接線変位を零へ戻す一行;

// 解と行の勾配を外へ出す口。いまこれを読むのは試験の場面と、連立を組み直して固有分解の内訳を見る計器だけである。
// 剛体どうしの本番の工程が入る便で試験の印を外す。
#[cfg(test)]
pub use row_gradient::一行の二つの符号付き勾配;
#[cfg(test)]
pub use solve_outcome::有効集合を組み替えて解いた結末;
#[cfg(test)]
pub use solution::接触点集合の法線と接線の同時解;
