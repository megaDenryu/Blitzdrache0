//! 多段シャドウ(CSM)の数学。カメラ視錐台を距離区分へ分け、距離区分ごとに光空間の正射影を組み立てる。
//! ここにあるのは純粋な計算だけであり、Vulkanの資源も描画の順序も知らない。
//! ashに触れないため、平坦な再エクスポートへ混ぜずモジュールごと公開し、型を`cascade::`で辿れるようにする。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「多段シャドウ(CSM)」

mod band;
mod band_mask;
mod blend;
mod bounding;
mod build;
mod camera_frustum;
mod cascade_set;
mod ortho;
mod ratio;
mod settings;
mod snap;
mod split;
mod texel_density;

#[cfg(test)]
mod cascade_tests;

/// 距離区分の数。設計正本が目標値として定めた4で確定している。
pub const 距離区分数: usize = 4;
/// 距離区分の境界の数。分割深度と遷移幅はこの数だけある。
pub const 距離区分の境界数: usize = 距離区分数 - 1;
/// 距離区分1枚あたりのシャドウマップの一辺。テクセルスナップの刻みはこの値と正射影の半幅から決まる。
pub const 距離区分の解像度: u32 = 2048;

pub use band::{距離区分の区間, 距離区分番号};
pub use band_mask::距離区分マスク;
pub use blend::{距離区分のブレンド, 距離区分を選ぶ};
pub use cascade_set::多段一式;
pub use ratio::{実用分割混合率, 距離区分の重なり率};
pub use settings::多段設定;
pub use texel_density::影の解像度密度;

pub(crate) use build::組み立てる;
