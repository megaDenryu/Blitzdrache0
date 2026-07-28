//! 個体が描かれるメッシュLOD段。0が最詳細であり、番号が増えるほど原型の粗い段になる。
//! 段そのものは上限を持たず順序だけを表す。何段まで存在するかは原型が持つメッシュの本数で決まるためである。
//! 地形詳細段と別の型にするのは、地形の段が束(チャンク)単位で選ばれるのに対し、この段が個体単位で選ばれ、
//! 上限の出どころも束のアセットでなく群の原型だからである。
//! 参照: `_doc/計画/ユビキタス言語.md`「個体詳細段」

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 個体詳細段(u8);

impl 個体詳細段 {
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
}
