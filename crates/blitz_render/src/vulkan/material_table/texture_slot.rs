//! 1つの資源表世代の中でのテクスチャの添字。担当するのは、世代を跨いで意味を持たない添字であることを型で表すことである。
//!
//! 不変条件: 値はその世代の画像集合の要素番号と1対1で対応する。発番の手段をこのモジュール木の内側に限るのは、
//! テクスチャ台帳だけがスロットを発番するという規律を可視性で守るためである。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct テクスチャスロット {
    値: u32,
}

impl テクスチャスロット {
    pub(in crate::vulkan::material_table) const fn 生成する(値: u32) -> Self {
        Self { 値 }
    }

    /// GPU境界: ディスクリプタ表を参照する生の添字へ戻す。
    pub(crate) const fn 添字(self) -> u32 {
        self.値
    }

    /// 世代が持つ画像集合を参照するための添字。要素数はスロットの発番数と等しいため、範囲の検証は世代の完成検査が持つ。
    pub(crate) fn 配列添字(self) -> usize {
        usize::try_from(self.値).unwrap_or_else(|_| panic!("テクスチャスロットがusizeに収まらない: {}", self.値))
    }
}
