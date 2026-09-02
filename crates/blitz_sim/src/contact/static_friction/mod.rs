//! 静止摩擦(判断13)。錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ補正を適用する。
//! 円錐の判定は接触点集合の単位であり、集合の全部の点を仮に射影してから接線の乗数の合力で1度だけ判定する。
//! 超えた接触点集合は補正を受けず、全部の錨を現在の接触点へ置き直して集合ごと滑走中と印す。抵抗は判断14の速度段階が受け持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

mod anchor;
#[cfg(test)]
mod isotropy_tests;
mod manifold_cone;
#[cfg(test)]
mod manifold_cone_tests;
mod projection;
#[cfg(test)]
mod projection_fixture;
#[cfg(test)]
mod projection_tests;
mod result;
mod substep_state;
mod tangential_multiplier;

pub use anchor::{剛体と静的世界の静止摩擦の錨, 剛体どうしの静止摩擦の錨, 静止摩擦の錨};
pub use manifold_cone::接触点集合の静止摩擦の仮の集計;
pub use result::静止摩擦の一回の仮の射影の結果;
pub use substep_state::静止摩擦の一細分の解の状態;
pub use tangential_multiplier::接線のラグランジュ乗数;
