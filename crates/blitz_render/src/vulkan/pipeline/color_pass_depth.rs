//! 色を描くパイプラインが深度をどう扱うかの状態。担当するのは、比べ方と書き込みの有無という2つを1つの値へ閉じ、
//! 深度プリパス方式からの写しをこの1箇所に持つことである。
//!
//! 2つを別々の引数で持ち回らないのは、成立する組み合わせが3つしか無いためである。等値で比べながら深度を書く組み合わせは、
//! 同じ値を書き直すだけの無駄であり、意味のある状態ではない。
//!
//! 布と粒子がこの型を通らないのは、どちらも深度プリパスの対象でないためである。プリパスが書いた深度に対して、布はより近いものを
//! 描いて書き、粒子は既存の深度と比べるだけであり、方式が変わっても扱いは変わらない。

use ash::vk;

use crate::frame_composition::深度プリパス方式;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum 色パスの深度状態 {
    より近いものを描いて書く,
    同値以下を描いて書く,
    等値だけを描いて書かない,
}

impl 色パスの深度状態 {
    pub(crate) const fn 方式から決める(方式: 深度プリパス方式) -> Self {
        match 方式 {
            深度プリパス方式::使わない => Self::より近いものを描いて書く,
            深度プリパス方式::使い色は同値以下で比べる => Self::同値以下を描いて書く,
            深度プリパス方式::使い色は等値で比べる => Self::等値だけを描いて書かない,
        }
    }

    pub(crate) const fn 比較(self) -> vk::CompareOp {
        match self {
            Self::より近いものを描いて書く => vk::CompareOp::LESS,
            Self::同値以下を描いて書く => vk::CompareOp::LESS_OR_EQUAL,
            Self::等値だけを描いて書かない => vk::CompareOp::EQUAL,
        }
    }

    pub(crate) const fn 書き込むか(self) -> bool {
        match self {
            Self::より近いものを描いて書く | Self::同値以下を描いて書く => true,
            Self::等値だけを描いて書かない => false,
        }
    }
}
