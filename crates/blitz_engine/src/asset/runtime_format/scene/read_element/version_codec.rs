//! 版ごとに違う要素の読み方を1つにまとめた値。担当するのは、テクスチャ1件の読み方と材質1件の読み方と静的物理形状の節の読み方が
//! 必ず同じ版から同時に決まることである。
//!
//! 3つを別々の引数で配らないのは、版6の材質の読み方へ版4のテクスチャの読み方を渡す組み合わせが、
//! 呼び出し側の書き間違いだけで成立してしまうためである。

use super::material_kind::{材質1件を読む工程, 版5までの材質1件を読む, 版6の材質1件を読む};
use super::static_shape::{版6までの静的物理形状を読む, 版7の静的物理形状を読む, 静的物理形状の節を読む工程};
use super::texture::{テクスチャを読む工程, 版4までのテクスチャを読む, 版5のテクスチャを読む};

#[derive(Clone, Copy)]
pub(in crate::asset::runtime_format::scene) struct 版ごとの要素の読み方 {
    テクスチャ1件: テクスチャを読む工程,
    材質1件: 材質1件を読む工程,
    静的物理形状の節: 静的物理形状の節を読む工程,
}

impl 版ごとの要素の読み方 {
    pub(in crate::asset::runtime_format::scene) const fn 版4() -> Self {
        Self {
            テクスチャ1件: 版4までのテクスチャを読む,
            材質1件: 版5までの材質1件を読む,
            静的物理形状の節: 版6までの静的物理形状を読む,
        }
    }

    pub(in crate::asset::runtime_format::scene) const fn 版5() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを読む,
            材質1件: 版5までの材質1件を読む,
            静的物理形状の節: 版6までの静的物理形状を読む,
        }
    }

    pub(in crate::asset::runtime_format::scene) const fn 版6() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを読む,
            材質1件: 版6の材質1件を読む,
            静的物理形状の節: 版6までの静的物理形状を読む,
        }
    }

    pub(in crate::asset::runtime_format::scene) const fn 版7() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを読む,
            材質1件: 版6の材質1件を読む,
            静的物理形状の節: 版7の静的物理形状を読む,
        }
    }

    pub(super) const fn テクスチャ1件(self) -> テクスチャを読む工程 {
        self.テクスチャ1件
    }

    pub(super) const fn 材質1件(self) -> 材質1件を読む工程 {
        self.材質1件
    }

    pub(in crate::asset::runtime_format::scene) const fn 静的物理形状の節(self) -> 静的物理形状の節を読む工程 {
        self.静的物理形状の節
    }
}
