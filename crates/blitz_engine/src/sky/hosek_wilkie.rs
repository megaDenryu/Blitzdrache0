//! Hosek-Wilkie解析近似の係数評価。時刻から決まる濁度・太陽高度と、シーンが持つ地表アルベドを受け取り、
//! 方向ごとの放射輝度式が使う係数を返す。方向ごとの評価そのものは空パスのシェーダーが行い、この層は係数までを担う。
//!
//! 参照: Lukas Hosek and Alexander Wilkie, "An Analytic Model for Full Spectral Sky-Dome Radiance" (SIGGRAPH 2012)。
//! データセットの出典と焼き方は`dataset`の冒頭にある。
//! 注意: 太陽高度が0未満の領域は近似の定義域外であり、この層は外挿しない(呼び出し元が0で頭打ちにしてから渡す)。

mod bezier;
mod configuration;
mod dataset;

pub(in crate::sky) use configuration::{放射輝度スケールを焼く, 方向係数を焼く};
pub(in crate::sky) use dataset::{チャネル数, 係数数};
