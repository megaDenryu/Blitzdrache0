//! そのフレームがライティングを行うかどうかの判別。担当するのは真偽1点である。
//!
//! 注意: 無効はシーンの画素段がベースカラーをそのまま返す特別な枝であり、「光が0件」とは別の状態である。
//! 光を0件にして同じ絵になると読み替えてはならない(環境光も影も掛からない絵になるためである)。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 照明の有効性 {
    有効,
    無効,
}

impl 照明の有効性 {
    pub(crate) fn 真偽から写す(値: bool) -> Self {
        if 値 { Self::有効 } else { Self::無効 }
    }

    pub(crate) fn 有効か(self) -> bool {
        matches!(self, Self::有効)
    }
}
