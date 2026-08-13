//! 実行時世界がどの生成規則と乱数の種から焼かれたかを表す由来。

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 世界の乱数の種(u32);

impl 世界の乱数の種 {
    pub const fn 生成する(値: u32) -> Self {
        Self(値)
    }

    pub const fn 値(self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 世界の生成器の版(u32);

impl 世界の生成器の版 {
    pub const fn 生成する(値: u32) -> Self {
        assert!(値 > 0, "世界の生成器の版は1以上でなければならない");
        Self(値)
    }

    pub const fn 値(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum 世界の由来 {
    種から生成した {
        乱数の種: 世界の乱数の種,
        生成器の版: 世界の生成器の版,
    },
    #[default]
    生成による由来を持たない,
}
