//! 焼き上げた表をCPU側で保持して補間で引く型を所有するモジュール。担当するのは、表の中身の保持と、任意の条件に対する
//! 双一次補間の標本取りである。
//!
//! GPUのサンプラーと同じ補間をCPUで持つのは、後段の積分(多重散乱・スカイビュー・空中遠近)が前段の表を引くため、
//! 引き方まで揃えないと積分の結果がGPUと食い違うからである。補間そのものは`bilinear`がこのモジュールの内側で持つ。

mod bilinear;
pub(in crate::atmosphere) mod multiscatter_table;
pub(in crate::atmosphere) mod skyview_table;
pub(in crate::atmosphere) mod transmittance_table;
