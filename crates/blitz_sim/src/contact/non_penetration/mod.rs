//! 非貫通(判断12)。法線方向の片側のXPBD位置拘束であり、乗数が0の接触点では隔たりが負のとき(貫通しているとき)だけ効く。
//! 乗数が正の接触点は隔たりが0以上でも式を評価し、負の増分で乗数を解放する(片側拘束の有効集合)。
//! 1細分あたりの補正の長さを形の最小の厚みで切り詰め、深い初期重なりを数細分に分けて回復させる。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断12: 非貫通は法線方向の片側のXPBD位置拘束であり、貫通しているときだけ効く」

#[cfg(test)]
mod applied_separation_fixture;
#[cfg(test)]
mod applied_separation_tests;
mod coefficients;
#[cfg(test)]
mod correction_tests;
#[cfg(test)]
pub(in crate::contact) mod fixture;
#[cfg(test)]
mod negative_increment_budget_tests;
mod projection;
#[cfg(test)]
mod projection_tests;
mod result;
mod substep_state;

pub use coefficients::{非貫通の一刻みの係数, 非貫通の解き方};
pub use result::非貫通の一回の射影の結果;
pub use substep_state::非貫通の一細分の解の状態;
