//! アセットが持つ画像本体の安定した識別子。担当するのは、別々のテクスチャIDが同じ画像を指しているかの判定材料を持つことである。
//!
//! テクスチャIDと別の型にするのは、標本化の設定だけが違う複数のテクスチャが同じ画像を指すアセット形式があるためである。
//! 画像IDが同じテクスチャは1つのスロットへ重複除去し、同じ画像を世代の中へ何枚も常駐させない。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct 画像ID {
    値: u64,
}

impl 画像ID {
    pub(crate) const fn 生成する(値: u64) -> Self {
        Self { 値 }
    }
}
