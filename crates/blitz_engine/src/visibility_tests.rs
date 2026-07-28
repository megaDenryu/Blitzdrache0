//! 可視判定の境界値検査。粗い判定で群ごと棄却されること、平面をまたぐ群で個体別に内外が分かれること、
//! 境界球が平面へ接する個体を描く側へ倒すこと、カメラ後方の個体を棄却することを確かめる。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」

#![allow(clippy::unwrap_used)]

mod fixture;
mod placement;
mod plane_boundary;
mod selection;
