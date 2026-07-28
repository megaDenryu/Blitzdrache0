//! 可視判定。毎フレーム、インスタンス群の個体のうちどれをシーンパスが描くかを、カメラ相対空間の視錐台と境界の交差で決める。
//! 判断はファイルにもGPUにも触れず、群の包囲領域による粗い判定を通った群だけを個体の境界球で細かく判定する。
//! 判定は保守的である。誤って描くこと(偽陽性)は許すが、誤って消すこと(偽陰性)は許さない。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」

mod bounds_transform;
mod count;
mod error;
mod frustum;
mod group_material;
mod instance_sphere;
mod plane;
mod select;
mod volume_test;

pub use count::可視判定計数;
pub use error::可視判定エラー;
pub use frustum::視錐台;
pub use group_material::群可視材料;
pub use instance_sphere::個体境界球;
pub use select::可視個体を選ぶ;
