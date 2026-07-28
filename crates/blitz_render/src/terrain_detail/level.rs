//! 地形の詳細段。0が最詳細であり、番号が増えるほど格子の刻みが粗くなる。
//! 段そのものは上限を持たず順序だけを表す。何段まで存在するかはアセットが焼いたメッシュの本数で決まるためである。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 地形詳細段(u8);

impl 地形詳細段 {
    pub const fn 最詳細() -> Self {
        Self(0)
    }

    pub const fn 番号から生成する(番号: u8) -> Self {
        Self(番号)
    }

    pub fn 番号(self) -> u8 {
        self.0
    }

    /// 段番号順に並んだメッシュ列を引くための添字。段番号と添字は同じ値である。
    pub fn 添字(self) -> usize {
        usize::from(self.0)
    }

    /// 1段粗い側。u8の上限に達していたら`None`を返す。飽和させると別の段と同じ値になり、順序の意味が壊れるためである。
    pub fn 一つ粗い段(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    /// 1段細かい側。最詳細より細かい段は存在しないため`None`を返す。
    pub fn 一つ細かい段(self) -> Option<Self> {
        self.0.checked_sub(1).map(Self)
    }
}
