//! 縁の一致: 隣り合うチャンクの高さ格子が、重なる帯の全ての格子点で同じ高さを持つことを確かめる検査。
//! 高さ場へ貼り合わせる経路と、地形メッシュを焼く経路の両方がこの検査を通る。
//!
//! 帯1組の突き合わせは`band`、チャンクの集まり全体の走査は`pairwise`、ソースファイルから読んで検査する入口は
//! `source_check`が持つ。
//! 参照: `_doc/設計/大規模世界の生成と遠景.md`、`_doc/設計/地形とカメラ相対描画.md`

mod band;
#[cfg(test)]
mod edge_agreement_tests;
mod error;
mod pairwise;
mod source_check;

pub use error::縁の一致エラー;
pub use pairwise::隣り合うチャンクの重なり帯が一致することを確かめる;
pub use source_check::ソースの高さ格子の重なり帯が一致することを確かめる;
