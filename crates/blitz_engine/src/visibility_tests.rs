//! 可視判定の境界値検査。粗い判定で群ごと棄却されること、平面をまたぐ群で個体別に内外が分かれること、
//! 境界球が平面へ接する個体を描く側へ倒すこと、カメラ後方の個体を棄却すること、
//! シーンとシャドウの2つの視錐台が4区分の並べ替えを作ることを確かめる。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」

#![allow(clippy::unwrap_used)]

mod fixture;
mod light_frustum;
mod placement;
mod plane_boundary;
mod quadrant;
mod run;
mod selection;
