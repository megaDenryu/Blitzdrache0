//! 接触の速度段階(判断14)。細分の速度の再構成の後に、接触点ごとの相対速度から反発と動摩擦を衝撃として1箇所ずつ適用する。
//! 位置と姿勢を書き換える口をどの型も持たず、書き換えられるのは速度と角速度だけである。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断14: 反発と動摩擦は速度段階で衝撃として1箇所ずつ適用し、低速の反発は零に落とす」

mod condition;
mod dynamic_friction;
#[cfg(test)]
mod impulse_fixture;
#[cfg(test)]
mod impulse_tests;
mod participant;
#[cfg(test)]
mod restitution_tests;
mod result;
mod stage;

pub use condition::速度段階の接触点の条件;
pub use participant::速度段階の参加点;
pub use result::接触の速度段階の結果;
pub use stage::{接触の速度段階, 接触点の法線の相対速度を求める};
