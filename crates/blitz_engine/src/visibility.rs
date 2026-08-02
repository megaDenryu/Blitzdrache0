//! 可視判定と個体別LODの選択。毎フレーム、インスタンス群の個体のうちどれをシーンパスと距離区分ごとのシャドウパスが
//! それぞれ描くかをカメラ相対空間の境界の交差で決め、同じ走査で個体ごとのメッシュLOD段を距離から決める。
//! シーンはカメラ視錐台の平面、影は距離区分ごとの正射影を光空間の直方体として見た区間で別々に判定する。
//! カメラの可視集合を影へ流用してはならない。
//! 判断はファイルにもGPUにも触れず、群の包囲領域による粗い判定を通った群だけを個体の境界球で細かく判定する。
//! 判定は保守的である。誤って描くこと(偽陽性)は許すが、誤って消すこと(偽陰性)は許さない。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」「個体別LOD」

mod band_light_box;
mod band_pass_test;
mod bounds_transform;
mod camera_distance;
mod caster_distance;
mod caster_distance_scan;
mod combined_select;
mod count;
mod error;
mod frustum;
mod group_material;
mod instance_sphere;
mod light_space_interval;
mod pass_test;
mod plane;
mod stage_bucket;
mod stage_tray;
mod visible_pass_set;
mod volume_test;

pub use band_light_box::距離区分別の光空間直方体;
pub use band_pass_test::距離区分別の可視判定;
pub use caster_distance::キャスター距離分布;
pub use caster_distance_scan::キャスターのカメラ距離分布を数える;
pub use combined_select::{個体選択材料, 可視個体と段を選ぶ, 段選択方式};
pub use count::可視判定計数;
pub use error::可視判定エラー;
pub use frustum::視錐台;
pub use group_material::群可視材料;
pub use instance_sphere::個体境界球;
pub use pass_test::可視判定方式;
pub use stage_tray::段別ID受け皿;
