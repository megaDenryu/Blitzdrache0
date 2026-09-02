//! 力の単位型。質量と加速度の積の次元を持ち、力のベクトル(`力`)の成分と大きさがこの型である。

/// ニュートン単位の力の大きさ。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ニュートン(f32);

impl ニュートン {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}
