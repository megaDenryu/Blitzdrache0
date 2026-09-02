//! 剛体の識別子: 台帳が発行する全順序を持つ番号。接触の履歴の鍵と島の順序がこの順序で並ぶ(判断16・17)。
//! u32のまま配ると点添字や静的世界の接触相手の識別子と取り違えても型が通るため、別の型で包む。

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 剛体の識別子(u32);

impl 剛体の識別子 {
    pub(super) fn 生成する(番号: u32) -> Self {
        Self(番号)
    }

    /// GPUのバイト列へ写す境界向けの生値。
    pub fn 値(&self) -> u32 {
        self.0
    }

    /// 台帳の一覧と参照計算の作業域の添字。破棄が無い本便では識別子の番号がそのまま添字である。
    pub(crate) fn 配列添字(&self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("剛体の識別子がusizeに収まらない: {}", self.0))
    }
}
