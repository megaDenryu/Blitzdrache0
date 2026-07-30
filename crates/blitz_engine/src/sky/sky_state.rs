//! 時刻から導いた天空の状態一式。

use blitz_math::ラジアン;

use super::{太陽光強度, 太陽光色, 太陽方向, 影の有効性, 昼係数, 環境光強度, 露出補正段};

/// 世界時刻と空と太陽の方針だけから決まる、そのフレームの天空の状態。
/// 各フィールドの型が自身の値域を検証済みであるため、この型は組み立てで追加の検査を持たない。
/// 組み立てを同じモジュールの導出工程だけに許すのは、フィールドどうしの整合(強度が0の時刻に影が有効にならない等)を
/// 1つの導出経路に閉じるためである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 天空状態 {
    pub(super) 太陽方向: 太陽方向,
    pub(super) 太陽高度: ラジアン,
    pub(super) 方向光色: 太陽光色,
    pub(super) 方向光強度: 太陽光強度,
    pub(super) 環境光強度: 環境光強度,
    pub(super) 露出補正段: 露出補正段,
    pub(super) 影の有効性: 影の有効性,
    pub(super) 昼係数: 昼係数,
}

impl 天空状態 {
    pub fn 太陽方向(&self) -> 太陽方向 {
        self.太陽方向
    }

    pub fn 太陽高度(&self) -> ラジアン {
        self.太陽高度
    }

    pub fn 方向光色(&self) -> 太陽光色 {
        self.方向光色
    }

    pub fn 方向光強度(&self) -> 太陽光強度 {
        self.方向光強度
    }

    pub fn 環境光強度(&self) -> 環境光強度 {
        self.環境光強度
    }

    pub fn 露出補正段(&self) -> 露出補正段 {
        self.露出補正段
    }

    pub fn 影の有効性(&self) -> 影の有効性 {
        self.影の有効性
    }

    pub fn 昼係数(&self) -> 昼係数 {
        self.昼係数
    }
}
