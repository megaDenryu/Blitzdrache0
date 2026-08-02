//! 1つの資源表世代の材質レコード列の中での位置。担当するのは、世代を跨いで意味を持たない添字であることを型で表すことである。
//!
//! 注意: この添字を単体で持ち回してはならない。どの世代の何番目かが揃って初めて材質が定まるため、
//! 世代を跨ぐ受け渡しには`材質GPU参照`を使い、生の添字を取り出すのは束縛する世代を確かめた後にする。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct 材質レコード添字 {
    値: u32,
}

impl 材質レコード添字 {
    pub(in crate::vulkan::material_table) const fn 生成する(値: u32) -> Self {
        Self { 値 }
    }

    /// GPU境界: 描画定数へ載せる生の添字へ戻す。
    pub(crate) const fn 添字(self) -> u32 {
        self.値
    }

    pub(crate) fn 配列添字(self) -> usize {
        usize::try_from(self.値).unwrap_or_else(|_| panic!("材質レコード添字がusizeに収まらない: {}", self.値))
    }
}
