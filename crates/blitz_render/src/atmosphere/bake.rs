//! ベイク済み画像1枚を全テクセル走査して焼き上げる工程を所有するモジュール。担当するのは、テクセルの添字1組から値を求めることと、
//! 解像度ぶんを走査して行優先の並びを返すことである。
//!
//! ここが焼く並びは、GPUが焼いた画像を行優先で読み戻した並びと一致する。この一致が読み戻し検査の前提であり、
//! 走査の順序を積分の中へ埋め込まずここへ集めるのは、期待値の並びという1つの契約を1箇所に置くためである。
//! 1枚を焼くのに要る大気以外の条件(観測高度と太陽の向き)もこのモジュールが型として持つ。

pub(in crate::atmosphere) mod aerial_condition;
pub(in crate::atmosphere) mod aerial_lut;
pub(in crate::atmosphere) mod multiscatter_lut;
pub(in crate::atmosphere) mod skyview_condition;
pub(in crate::atmosphere) mod skyview_lut;
pub(in crate::atmosphere) mod transmittance_lut;
