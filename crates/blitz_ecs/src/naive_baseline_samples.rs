//! 素朴な置き場の計測と正しさの試験が入れる、見本の個体構成要素。位置のように倍精度の3つ組を持つ型と、
//! 数と真偽だけの小さな状態の型の2つを置く。大きさの違う2種類を測るためであり、具体ゲームの型ではない。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断8」。

use crate::component_marker::個体構成要素;

/// 見本の位置。ゲーム上の意味を持たず、置き場に入る値の大きさを倍精度の3つ組で代表する。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 見本の位置 {
    成分: [f64; 3],
}

impl 個体構成要素 for 見本の位置 {}

impl 見本の位置 {
    pub(crate) fn 生成する(成分: [f64; 3]) -> Self {
        Self { 成分 }
    }

    /// 走査が値を実際に読むことを保証するための足し合わせ。
    pub(crate) fn 成分の和(&self) -> f64 {
        self.成分.iter().sum()
    }
}

/// 見本の小さな状態。置き場に入る値が小さいときの走査を代表する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 見本の小さな状態 {
    残り体力: u32,
    地面に接しているか: bool,
}

impl 個体構成要素 for 見本の小さな状態 {}

impl 見本の小さな状態 {
    pub(crate) fn 生成する(残り体力: u32, 地面に接しているか: bool) -> Self {
        Self { 残り体力, 地面に接しているか }
    }

    pub(crate) fn 残り体力(&self) -> u32 {
        self.残り体力
    }

    pub(crate) fn 地面に接しているか(&self) -> bool {
        self.地面に接しているか
    }
}
