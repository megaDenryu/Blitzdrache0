//! 密度場で散らす様式の検査。担当するのは、決定性・密度の効き方・傾きの好み・チャンクの内側に収まることと接地の
//! 4つを、宣言を1件も置いていない段階でも確かめることである。
//!
//! 材料の組み立ては`fixture`が持ち、検査は見るものごとにファイルを分ける。

#![allow(clippy::unwrap_used)]

mod bounds_and_grounding;
mod density;
mod determinism;
mod fixture;
mod slope;
