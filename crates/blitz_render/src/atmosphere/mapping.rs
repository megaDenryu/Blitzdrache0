//! LUTのテクセル座標と物理条件の間の写像を所有するモジュール。担当するのは、各LUTの解像度・テクセルの添字と単位位置の
//! 行き来・条件(半径と余弦の組)と単位位置の行き来である。
//!
//! 焼く側と引く側が同じ写像を通ることをこのモジュール1つが保証する。写像を積分の中へ散らすと、焼いた座標と引く座標が
//! 食い違って色が静かにずれるという、絵にしか現れない欠陥になる。往復が恒等であることは`atmosphere_tests`が検査する。

pub(in crate::atmosphere) mod aerial_mapping;
pub(in crate::atmosphere) mod lut_resolution;
pub(in crate::atmosphere) mod multiscatter_mapping;
pub(in crate::atmosphere) mod skyview_lookup;
pub(in crate::atmosphere) mod skyview_mapping;
pub(in crate::atmosphere) mod transmittance_mapping;
pub(in crate::atmosphere) mod unit_texel;
