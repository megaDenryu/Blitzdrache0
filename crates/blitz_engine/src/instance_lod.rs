//! 個体別のLOD選択。インスタンス群の個体1件ごとに、カメラまでの距離とヒステリシスでメッシュLOD段を決める。
//! 群単位でなく個体単位で選ぶのは、一辺100メートルのチャンクで近端と遠端の個体を同じ段にすると距離別LODとして粗すぎ、
//! カメラ移動時に群全体が同時に切り替わって目立つためである。
//! 閾値とヒステリシスの規則は`lod_threshold`の核を地形と共有する。隣接差1の緩和は持ち込まない。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「個体別LOD」

mod error;
mod level;
mod settings;
mod state;

pub use error::個体LODエラー;
pub use level::個体詳細段;
pub use settings::個体LOD選択設定;
pub use state::個体別段状態;
