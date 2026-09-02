//! 回転衝撃の単位型。回転力と時間の積(角運動量と同じ次元)を持ち、回転衝撃のベクトル(`回転衝撃`)の成分と大きさがこの型である。
//! 剛体の角運動量の大きさもこの型で読み、自由回転で発散しないことの検査がそれを比べる。

/// ニュートンメートル秒単位の回転衝撃(角運動量)の大きさ。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ニュートンメートル秒(f32);

impl ニュートンメートル秒 {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }

    /// 同じ次元どうしの比。無次元量になる。
    pub fn 比(&self, 分母: Self) -> f32 {
        self.0 / 分母.0
    }
}
