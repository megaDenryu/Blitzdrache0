//! 実行時世界がどの生成規則と乱数の種から焼かれたかを表す由来。

use super::runtime_format::アセット実行時形式エラー;

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
    pub const fn 生成する(値: u32) -> Result<Self, アセット実行時形式エラー> {
        if 値 == 0 {
            Err(アセット実行時形式エラー::世界の由来が不正("生成器の版が0"))
        } else {
            Ok(Self(値))
        }
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
