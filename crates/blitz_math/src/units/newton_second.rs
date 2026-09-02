//! 衝撃の単位型。力と時間の積(運動量と同じ次元)を持ち、衝撃のベクトル(`衝撃`)の成分と大きさがこの型である。

/// ニュートン秒単位の衝撃の大きさ。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ニュートン秒(f32);

impl ニュートン秒 {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}
