//! 可視判定の境界値検査。粗い判定で群ごと棄却されること、平面をまたぐ群で個体別に内外が分かれること、
//! 境界球が平面へ接する個体を描く側へ倒すこと、カメラ後方の個体を棄却すること、
//! カメラ視錐台と帯ごとの光空間直方体がパス別の区間を作ることを確かめる。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」、
//! `_doc/設計/空と時間帯と遠距離シャドウ.md`「可視ID列の帯別区間(4区分並べの後継)」

#![allow(clippy::unwrap_used)]

mod fixture;
mod light_box;
mod light_box_degenerate;
mod light_box_radius;
mod pass_interval;
mod placement;
mod plane_boundary;
mod run;
mod selection;
