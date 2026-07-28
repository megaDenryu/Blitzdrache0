//! インスタンス群を構成する値の集まり。原型・個体配置・境界がひとつの不変条件を分担するため、同じモジュール木に置く。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「インスタンス群の表現」

mod archetype;
mod bounding_box;
mod bounding_sphere;
mod bounds;
mod error;
mod group;
mod placement;
#[cfg(test)]
mod placement_tests;

pub use archetype::原型;
pub use bounding_box::軸平行包囲領域;
pub use bounding_sphere::境界球;
pub use bounds::群境界;
pub use error::インスタンス群エラー;
pub use group::インスタンス群;
pub use placement::{チャンクアンカーからの許容メートル, 個体配置};
