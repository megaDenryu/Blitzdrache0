//! チャンク: 世界のストリーミング単位を一意に決める座標。
//! アセット層の所有チャンクとストリーミング層の格子・台帳が同じ識別子を共有するため、どちらの下にも置かず独立させる。

mod chunk_coordinate;
#[cfg(test)]
mod chunk_coordinate_tests;

pub use chunk_coordinate::チャンク座標;
