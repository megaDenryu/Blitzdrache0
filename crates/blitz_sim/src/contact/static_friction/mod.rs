//! 静止摩擦(判断13)。錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ補正を適用する。
//! 接線は接触点集合1つを1つの連立として解き、その連立へは非貫通を解いた配置の法線の行も同じ行列で入る
//! (連立そのものは `normal_tangential_system` が持つ)。点ごとに順に射影すると、各点が物体全体の滑りを1点で
//! 戻そうとして接線の乗数が積み増され、傾きの正接が静止摩擦係数より小さい坂でも円錐を超えて滑走へ落ちる。
//! 法線の行を同じ連立へ入れるのは、接線だけを解くと接触点に働く接線の力が回転を含む補正を作り、次の反復の
//! 非貫通がその回転を戻して錨の接線変位を作り直すためである。
//! 円錐の判定は接触点集合の単位であり、集合の全部の点の仮の配置と仮の乗数が揃ってから合力で1度だけ判定する。
//! 超えた接触点集合は補正を受けず、全部の錨を現在の接触点へ置き直して集合ごと滑走中と印す。抵抗は判断14の速度段階が受け持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

mod anchor;
#[cfg(test)]
mod isotropy_tests;
mod manifold_cone;
#[cfg(test)]
mod manifold_cone_tests;
mod manifold_row_build;
mod manifold_row_result;
mod manifold_tentative;
mod projection;
#[cfg(test)]
mod projection_fixture;
#[cfg(test)]
mod projection_tests;
mod result;
mod substep_state;
mod tangential_multiplier;
mod tentative_point_multiplier;

pub use anchor::{剛体と静的世界の静止摩擦の錨, 剛体どうしの静止摩擦の錨, 静止摩擦の錨};
pub use manifold_cone::接触点集合の静止摩擦の仮の集計;
pub use manifold_row_result::静止摩擦の連立へ点を入れた結果;
pub use manifold_tentative::接触点集合の仮の乗数の集まり;
pub use result::静止摩擦の一回の仮の射影の結果;
pub use substep_state::静止摩擦の一細分の解の状態;
pub use tangential_multiplier::接線のラグランジュ乗数;
pub use tentative_point_multiplier::接触点の仮の接線の乗数;
