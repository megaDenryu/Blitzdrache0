//! 問い合わせ件数: 1刻みの中で世界の形を尋ねる口へ渡した問いの数。性能予算(1刻み最大7件)と突き合わせる計器の値である。

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 問い合わせ件数(u32);

impl 問い合わせ件数 {
    pub fn 零() -> Self {
        Self(0)
    }

    pub fn 一件数える(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn 値(self) -> u32 {
        self.0
    }
}
