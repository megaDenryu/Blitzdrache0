//! 静穏の連続細分数: 動的剛体が連続して静穏である細分の数(判断18)。島の全剛体でこの数が閾値に達すると島が休止する。
//! 休止の判定そのものはIssue #40が実装し、この作業は動的の枝が持つ状態の型だけを置く。

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 静穏の連続細分数(u32);

impl 静穏の連続細分数 {
    /// 起きた直後、または静穏でない細分を挟んだ直後の値。
    pub fn 零() -> Self {
        Self(0)
    }

    pub fn 値(&self) -> u32 {
        self.0
    }

    /// 静穏な細分を1つ通過したときの値。
    pub fn 一つ進める(&self) -> Self {
        Self(self.0.saturating_add(1))
    }
}
