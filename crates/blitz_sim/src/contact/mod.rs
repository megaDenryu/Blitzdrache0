//! 剛体力学層のうち接触(判断1の`contact/`)。接触物性と混合則(判断15)・接触拘束の2つのバッチとその生成(判断11)・
//! 非貫通の片側XPBD位置拘束(判断12)を持つ。`blitz_collision` の接触点集合を読むのはこのモジュールだけである。
//! 静止摩擦(判断13)・速度段階(判断14)・接触の履歴(判断16)・接触島(判断17)・休止(判断18)・細分の工程(判断19)は後続の便で足す。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断11: 接触拘束は接触点集合から細分ごとに生成する一時のバッチであり、参加者の組ごとに別のバッチを持ち、座標系の写しは剛体の側が行う」

mod batch_builder;
mod batch_builder_push;
#[cfg(test)]
mod batch_generation_static_tests;
#[cfg(test)]
mod batch_generation_tests;
mod body_body_contact;
mod body_body_contact_parameters;
mod body_static_contact;
mod body_static_contact_parameters;
mod contact_batches;
mod contact_property;
#[cfg(test)]
mod contact_test_fixtures;
mod contacting_body;
mod feature_identity;
mod friction_coefficient;
mod generation_error;
mod material_id;
mod material_pair;
mod minimum_thickness;
mod mixing_rule;
mod mixing_rule_builder;
#[cfg(test)]
mod mixing_rule_builder_tests;
#[cfg(test)]
mod mixing_rule_tests;
mod non_penetration_coefficients;
#[cfg(test)]
mod non_penetration_correction_tests;
#[cfg(test)]
mod non_penetration_fixture;
mod non_penetration_participant;
mod non_penetration_projection;
mod non_penetration_result;
#[cfg(test)]
mod non_penetration_tests;
mod penetration_depth;
mod property_error;
#[cfg(test)]
mod property_test_fixtures;
mod restitution_coefficient;
mod shared_frame;
#[cfg(test)]
mod stacked_box_fixture;
mod static_world_partner;
mod static_world_partner_id;
mod surface_property;

pub use batch_builder::接触拘束のバッチの組み立て;
pub use body_body_contact::剛体と剛体の接触拘束;
pub use body_body_contact_parameters::剛体と剛体の接触拘束の引数;
pub use body_static_contact::剛体と静的世界の接触拘束;
pub use body_static_contact_parameters::剛体と静的世界の接触拘束の引数;
pub use contact_batches::接触拘束の二つのバッチ;
pub use contact_property::接触物性;
pub use contacting_body::接触に参加する剛体;
pub use feature_identity::接触の特徴の識別;
pub use friction_coefficient::摩擦係数;
pub use generation_error::接触拘束の生成エラー;
pub use material_id::材質の識別子;
pub use material_pair::材質の対;
pub use minimum_thickness::形の最小の厚み;
pub use mixing_rule::混合則;
pub use mixing_rule_builder::混合則の組み立て;
pub use non_penetration_coefficients::{非貫通の一刻みの係数, 非貫通の解き方};
pub use non_penetration_participant::非貫通の参加点;
pub use non_penetration_result::非貫通の一回の射影の結果;
pub use penetration_depth::貫通量;
pub use property_error::接触物性エラー;
pub use restitution_coefficient::反発係数;
pub use static_world_partner::静的世界の接触相手;
pub use static_world_partner_id::静的世界の接触相手の識別子;
pub use surface_property::表面物性;
