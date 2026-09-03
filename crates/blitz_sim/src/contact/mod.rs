//! 剛体力学層のうち接触(判断1の`contact/`)。接触物性と混合則(判断15)・接触拘束の2つのバッチとその生成(判断11)・
//! 非貫通の片側XPBD位置拘束(判断12の`non_penetration/`)・静止摩擦の位置拘束(判断13の`static_friction/`)・
//! 速度段階の反発と動摩擦(判断14の`velocity_stage/`)・接触の履歴(判断16の`history/`)を持つ。
//! `blitz_collision` の接触点集合を読むのはこのモジュールだけである。
//! 接触島(判断17)・休止(判断18)・細分の工程(判断19)は後続の便で足す。細分の工程が組み上がるまでの間、
//! 場面を進めて判断13と判断14の定量の基準を測るのは`scene/`の試験専用の材料である。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断11: 接触拘束は接触点集合から細分ごとに生成する一時のバッチであり、参加者の組ごとに別のバッチを持ち、座標系の写しは剛体の側が行う」

mod batch_builder;
mod batch_builder_push;
#[cfg(test)]
mod batch_generation_order_tests;
#[cfg(test)]
mod batch_generation_static_tests;
#[cfg(test)]
mod batch_generation_tests;
mod body_body_contact;
mod body_body_contact_parameters;
mod body_static_contact;
mod body_static_contact_parameters;
mod contact_batches;
mod contact_projection_participant;
mod contact_property;
mod contact_property_limits;
#[cfg(test)]
mod contact_test_fixtures;
mod contact_thresholds;
mod contacting_body;
mod feature_identity;
mod friction_coefficient;
mod generation_error;
mod history;
mod manifold_range;
mod material_id;
mod material_pair;
mod minimum_thickness;
mod mixing_rule;
mod mixing_rule_builder;
#[cfg(test)]
mod mixing_rule_builder_tests;
#[cfg(test)]
mod mixing_rule_tests;
mod non_penetration;
mod penetration_depth;
mod property_error;
#[cfg(test)]
mod property_test_fixtures;
mod restitution_coefficient;
#[cfg(test)]
mod scene;
mod shared_frame;
mod solver_quality;
mod solver_quality_error;
#[cfg(test)]
mod solver_quality_tests;
#[cfg(test)]
mod stacked_box_fixture;
mod static_friction;
mod static_world_partner;
mod static_world_partner_id;
mod surface_property;
mod symmetric_system;
mod velocity_stage;

pub use batch_builder::接触拘束のバッチの組み立て;
pub use body_body_contact::剛体と剛体の接触拘束;
pub use body_body_contact_parameters::剛体と剛体の接触拘束の引数;
pub use body_static_contact::剛体と静的世界の接触拘束;
pub use body_static_contact_parameters::剛体と静的世界の接触拘束の引数;
pub use contact_batches::接触拘束の二つのバッチ;
pub use contact_projection_participant::接触の射影に参加する点;
pub use contact_property::接触物性;
pub use contact_thresholds::{
    休止と判定する並進速度の閾値, 休止と判定する角速度の閾値, 休止と判定する連続細分数, 反発を抑制する法線相対速度の閾値
};
pub use contacting_body::接触に参加する剛体;
pub use feature_identity::接触の特徴の識別;
pub use friction_coefficient::摩擦係数;
pub use generation_error::接触拘束の生成エラー;
pub use history::{
    剛体と静的世界の接触の履歴, 剛体と静的世界の接触の履歴の項目, 剛体と静的世界の接触の継続の記録, 剛体と静的世界の接触の鍵, 剛体どうしの接触の履歴,
    剛体どうしの接触の履歴の項目, 剛体どうしの接触の継続の記録, 剛体どうしの接触の鍵, 接触の併走の結果, 接触の対応付け, 接触の履歴, 接触の履歴の項目,
    接触の履歴エラー, 接触の継続の記録,
};
pub use manifold_range::接触点集合の占める範囲;
pub use material_id::材質の識別子;
pub use material_pair::材質の対;
pub use minimum_thickness::形の最小の厚み;
pub use mixing_rule::混合則;
pub use mixing_rule_builder::混合則の組み立て;
pub use non_penetration::{
    接触点集合の法線の同時解, 非貫通の一刻みの係数, 非貫通の一回の射影の結果, 非貫通の一細分の解の状態, 非貫通の接触点集合の一行, 非貫通の解き方,
};
pub use penetration_depth::貫通量;
pub use property_error::接触物性エラー;
pub use restitution_coefficient::反発係数;
pub use solver_quality::接触を解く品質の設定;
pub use solver_quality_error::接触を解く品質の設定エラー;
pub use static_friction::{
    剛体と静的世界の静止摩擦の錨, 剛体どうしの静止摩擦の錨, 接線のラグランジュ乗数, 接触点の仮の接線の乗数, 接触点集合の静止摩擦の仮の集計,
    接触点集合の静止摩擦の同時解, 接触点集合の静止摩擦の連立, 静止摩擦の一回の仮の射影の結果, 静止摩擦の一細分の解の状態, 静止摩擦の接触点集合の一行,
    静止摩擦の連立へ点を入れた結果, 静止摩擦の錨,
};
pub use static_world_partner::静的世界の接触相手;
pub use static_world_partner_id::静的世界の接触相手の識別子;
pub use surface_property::表面物性;
pub use velocity_stage::{
    接触の速度段階, 接触の速度段階の結果, 接触点の法線の相対速度を求める, 速度段階の参加点, 速度段階の接触点の条件
};
