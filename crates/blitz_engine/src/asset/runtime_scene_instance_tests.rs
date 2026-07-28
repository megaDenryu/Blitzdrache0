//! 版3のインスタンス群に対する検査。破れを検出する地点が2つあるため、地点ごとに1つの子モジュールへ分ける。
//! `byte_boundary_tests`は書き出したバイト列の既知の位置を壊して読取が拒むことを、
//! `invariant_tests`は値の組み合わせが群の不変条件を破るときに生成が拒むことを見る。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「インスタンス群の表現」

#![allow(clippy::unwrap_used)]

mod byte_boundary_tests;
mod fixture;
mod invariant_tests;
