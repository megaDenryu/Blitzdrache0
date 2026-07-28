//! 地形LODの選択。毎フレーム、常駐しているチャンクごとにどの詳細段を描くかを決める。
//! 段の語彙と幾何はレンダラーとアセットが持ち、この層は「どれを描くか」の判断だけを持つ。判断はファイルにもGPUにも触れず、
//! 常駐していないチャンクを要求しない。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

mod distance;
mod error;
mod hysteresis;
mod relaxation;
#[cfg(test)]
mod relaxation_tests;
mod selection_settings;
mod selector;

pub use error::地形LODエラー;
pub use selection_settings::地形LOD選択設定;
pub use selector::地形LOD選択器;
