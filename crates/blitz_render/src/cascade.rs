//! カスケードシャドウ(CSM)の数学。カメラ視錐台を距離帯へ分け、帯ごとに光空間の正射影を組み立てる。
//! ここにあるのは純粋な計算だけであり、Vulkanの資源も描画の順序も知らない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「カスケードシャドウ(CSM)」

mod band;
mod blend;
mod bounding;
mod build;
mod camera_frustum;
mod cascade_set;
mod enclosing;
mod ortho;
mod ratio;
mod settings;
mod snap;
mod split;

#[cfg(test)]
mod cascade_tests;

/// 距離帯の数。設計正本が目標値として定めた4で確定している。
pub const 帯数: usize = 4;
/// 帯の境界の数。分割深度と遷移幅はこの数だけある。
pub const 帯境界数: usize = 帯数 - 1;
/// 帯1枚あたりのシャドウマップの一辺。テクセルスナップの刻みはこの値と正射影の半幅から決まる。
pub const 帯解像度: u32 = 2048;

pub use band::{帯区間, 帯番号};
pub use blend::{帯を選ぶ, 帯ブレンド};
pub use cascade_set::カスケード一式;
pub use ratio::{実用分割混合率, 帯重なり率};
pub use settings::カスケード設定;

pub(crate) use build::組み立てる;
