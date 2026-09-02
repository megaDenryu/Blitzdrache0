//! 目標拘束のバッチの1本を指す添字。点添字とも距離拘束の拘束添字とも別の型にするのは、介入が目標拘束を着脱するときに取り違えを型で止めるためである。

/// 目標拘束の添字。並びは`目標拘束のバッチ`の拘束一覧の順である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 目標拘束添字(u32);

impl 目標拘束添字 {
    pub fn 生成する(値: u32) -> Self {
        Self(値)
    }

    /// GPUのバイト列へ書く境界向けの生値。
    pub fn 値(&self) -> u32 {
        self.0
    }

    pub fn 配列添字(&self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("目標拘束添字がusizeに収まらない: {}", self.0))
    }
}
