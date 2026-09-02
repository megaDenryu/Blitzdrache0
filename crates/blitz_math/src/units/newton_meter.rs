//! 回転力の単位型。力と腕の長さの積の次元を持ち、回転力のベクトル(`回転力`)の成分と大きさがこの型である。

/// ニュートンメートル単位の回転力の大きさ。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ニュートンメートル(f32);

impl ニュートンメートル {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}
