//! 版ごとに違う要素の書き方を1つにまとめた値。読み取り側の`read_element::version_codec`と対になっており、
//! テクスチャ1件の書き方と材質1件の書き方と静的物理形状の節の書き方が必ず同じ版から同時に決まることを担当する。

#[cfg(test)]
use super::material_kind::版5までの材質1件を書く;
use super::material_kind::{材質1件を書く工程, 版6の材質1件を書く};
#[cfg(test)]
use super::static_shape::版6までの静的物理形状を書く;
use super::static_shape::{版7の静的物理形状を書く, 静的物理形状の節を書く工程};
#[cfg(test)]
use super::texture::版4までのテクスチャを書く;
use super::texture::{テクスチャを書く工程, 版5のテクスチャを書く};

#[derive(Clone, Copy)]
pub(in crate::asset::runtime_format::scene) struct 版ごとの要素の書き方 {
    テクスチャ1件: テクスチャを書く工程,
    材質1件: 材質1件を書く工程,
    静的物理形状の節: 静的物理形状の節を書く工程,
}

impl 版ごとの要素の書き方 {
    /// 版4で書き出す経路は検査だけが持つ。実行時形式の書き出しは常に最新版で行うためである。
    #[cfg(test)]
    pub(in crate::asset::runtime_format::scene) const fn 版4() -> Self {
        Self {
            テクスチャ1件: 版4までのテクスチャを書く,
            材質1件: 版5までの材質1件を書く,
            静的物理形状の節: 版6までの静的物理形状を書く,
        }
    }

    /// 版5で書き出す経路も検査だけが持つ。版5から版6への昇格が恒等であることを、この書き方で焼いた材料が示す。
    #[cfg(test)]
    pub(in crate::asset::runtime_format::scene) const fn 版5() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを書く,
            材質1件: 版5までの材質1件を書く,
            静的物理形状の節: 版6までの静的物理形状を書く,
        }
    }

    /// 版6で書き出す経路も検査だけが持つ。版6から版7への昇格が恒等であることを、この書き方で焼いた材料が示す。
    #[cfg(test)]
    pub(in crate::asset::runtime_format::scene) const fn 版6() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを書く,
            材質1件: 版6の材質1件を書く,
            静的物理形状の節: 版6までの静的物理形状を書く,
        }
    }

    pub(in crate::asset::runtime_format::scene) const fn 版7() -> Self {
        Self {
            テクスチャ1件: 版5のテクスチャを書く,
            材質1件: 版6の材質1件を書く,
            静的物理形状の節: 版7の静的物理形状を書く,
        }
    }

    pub(super) const fn テクスチャ1件(self) -> テクスチャを書く工程 {
        self.テクスチャ1件
    }

    pub(super) const fn 材質1件(self) -> 材質1件を書く工程 {
        self.材質1件
    }

    pub(in crate::asset::runtime_format::scene) const fn 静的物理形状の節(self) -> 静的物理形状の節を書く工程 {
        self.静的物理形状の節
    }
}
