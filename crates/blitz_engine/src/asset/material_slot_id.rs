//! 材質スロットID: 原型ローカルな材質の置き場所の番号。
//! glTFのmaterial indexは入力境界だけの番号であり、アセットコンパイラがこの番号へ正規化する。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「入力契約と実行時形式(scene v4)」

/// 1つの原型が持つスロットの語彙の中で材質の置き場所を区別する番号。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 材質スロットID(u32);

impl 材質スロットID {
    /// すべてのu32を有効な番号として受け入れる。どの番号が実在するかは材質集合が決めるため、この型は範囲を持たない。
    pub fn 生成する(番号: u32) -> Self {
        Self(番号)
    }

    /// 永続化・ログ境界で使う番号を返す。
    pub fn 番号を返す(self) -> u32 {
        self.0
    }
}
