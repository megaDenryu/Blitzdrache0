//! 剛体力学層のうち接触(判断1の`contact/`)。接触物性と混合則(判断15)・接触拘束の2つのバッチとその生成(判断11)・
//! 非貫通の片側XPBD位置拘束(判断12)を持つ。`blitz_collision` の接触点集合を読むのはこのモジュールだけである。
//! 静止摩擦(判断13)・速度段階(判断14)・接触の履歴(判断16)・接触島(判断17)・休止(判断18)・細分の工程(判断19)は後続の便で足す。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断11: 接触拘束は接触点集合から細分ごとに生成する一時のバッチであり、参加者の組ごとに別のバッチを持ち、座標系の写しは剛体の側が行う」

mod contact_property;
mod friction_coefficient;
mod material_id;
mod material_pair;
mod mixing_rule;
mod mixing_rule_builder;
#[cfg(test)]
mod mixing_rule_builder_tests;
#[cfg(test)]
mod mixing_rule_tests;
mod property_error;
#[cfg(test)]
mod property_test_fixtures;
mod restitution_coefficient;
mod surface_property;

pub use contact_property::接触物性;
pub use friction_coefficient::摩擦係数;
pub use material_id::材質の識別子;
pub use material_pair::材質の対;
pub use mixing_rule::混合則;
pub use mixing_rule_builder::混合則の組み立て;
pub use property_error::接触物性エラー;
pub use restitution_coefficient::反発係数;
pub use surface_property::表面物性;
