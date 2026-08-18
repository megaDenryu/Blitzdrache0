//! `cargo xtask`が人間の操作者へコマンド一覧を提示する層をまとめた束。担当するのは、静的な
//! 一覧表示(`usage`)・対話メニュー(`menu`)・両方が使う一覧の正本(`command_catalog`)という
//! 人向け提示の関心をまとめて所有し、外へ公開することだけである。3つの中身はそれぞれ独立した
//! 責務を持ち、この束自身は状態を持たない。

pub(crate) mod command_catalog;
pub(crate) mod menu;
pub(crate) mod usage;
