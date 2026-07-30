//! 経路に沿った積分を所有するモジュール。担当するのは、視線1本を刻んで光学距離・透過率・散乱の放射輝度を積む手順と、
//! 積分の各点で太陽が見えるかどうかの判定である。
//!
//! 刻み方(区間数・標本位置・終わりの決め方)がここに閉じているのは、その値がGPUの写しと一致していなければならない
//! 唯一の場所だからである。区間数のような定数は`cargo xtask conform`の定数一致検査が正本としてこのモジュールの行を読む。
//! 位相関数と太陽の可視性を同じ場所へ置くのは、どちらも「1つの標本点で源の項をいくら立てるか」の一部だからである。

pub(in crate::atmosphere) mod aerial_march;
pub(in crate::atmosphere) mod multiscatter_march;
pub(in crate::atmosphere) mod multiscatter_series;
pub(in crate::atmosphere) mod multiscatter_step;
pub(in crate::atmosphere) mod optical_length;
pub(in crate::atmosphere) mod phase;
pub(in crate::atmosphere) mod skyview_march;
pub(in crate::atmosphere) mod skyview_step;
pub mod sun_visibility;
pub(in crate::atmosphere) mod transmittance;
