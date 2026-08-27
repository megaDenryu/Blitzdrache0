//! シミュレーション基盤層(判断51): 手法の数学だけを持つ。
//! 布メッシュ生成・XPBD拘束データ構築・介入の型・GPUバッファレイアウト定義と、
//! テスト用のCPU参照計算。GPU実行(コンピュートパス)はblitz_renderが担う。
//!
//! 注意: このクレートはashにもblitz_renderにも依存しない(特定の絵も知らない)。
//! 参照: `_doc/設計/シミュレーション層.md`「3層構造」。

#![forbid(unsafe_code)]

mod cloth;
mod gpu_layout;
mod intervention;
mod sph;
mod surface_flow;
mod xpbd;

pub use cloth::{
    布を生成する, 布データ, 布仕様, 布生成エラー, 既定一辺粒子数, 空き添字, 粒子, 距離拘束, 隣接拘束エントリ
};
pub use gpu_layout::{拘束バイト列にする, 粒子バイト列にする, 表面流バイト列にする, 隣接拘束バイト列にする};
pub use intervention::{バイト列にする, 介入};
pub use sph::{Sph仕様, Sph仕様エラー, sphを一ステップ進める, sph密度を計算する, 流体粒子};
pub use surface_flow::{表面セル, 表面流仕様, 表面流仕様エラー, 表面流状態};
pub use xpbd::距離拘束を射影する;
