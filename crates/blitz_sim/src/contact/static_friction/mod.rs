//! 静止摩擦(判断13)。錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ補正を適用する。
//! 円錐を超えた接触点は補正を受けず、錨を現在の接触点へ置き直して滑走中と印す。抵抗は判断14の速度段階が受け持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断13: 静止摩擦は錨からの接線変位を零へ戻す位置拘束であり、クーロン円錐の内側でだけ効く」

mod anchor;
mod projection;
#[cfg(test)]
mod projection_tests;
mod result;
mod substep_state;
mod tangential_multiplier;

pub use anchor::{剛体と静的世界の静止摩擦の錨, 剛体どうしの静止摩擦の錨, 静止摩擦の錨};
pub use result::静止摩擦の一回の射影の結果;
pub use substep_state::静止摩擦の一細分の解の状態;
pub use tangential_multiplier::接線のラグランジュ乗数;
