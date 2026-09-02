//! 目標拘束のバッチの1本を指す添字の、GPU境界向けの写し。`blitz_sim::constraint_graph::目標拘束添字`と同じ値を運ぶが、
//! blitz_renderはblitz_simに依存しないため型を共有せず、`布の彩色の区間`と同じく値だけを写す(写すのはblitz_appの目標拘束を組む工程である)。
//! 生値へ落とすのは目標の更新対応のバイト列化(`vulkan::cloth::buffers`の目標拘束のバッファの生成)だけである。

use std::fmt;

/// 目標拘束の添字。並びは目標拘束の引数バイト列の拘束の順である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 布の目標拘束添字(u32);

impl 布の目標拘束添字 {
    /// blitz_simの目標拘束添字の値から写す。
    pub const fn 生成する(値: u32) -> Self {
        Self(値)
    }

    /// GPUのバイト列へ書く境界向けの生値。
    pub fn 値(self) -> u32 {
        self.0
    }

    pub(super) fn 配列添字(self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("布の目標拘束添字{}がusizeに収まらない", self.0))
    }
}

impl fmt::Display for 布の目標拘束添字 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.0)
    }
}
