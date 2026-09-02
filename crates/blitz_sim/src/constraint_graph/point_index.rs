//! 拘束グラフの点自由度を指す添字。u32のまま配ると拘束の添字と取り違えても型が通るため、別の型で包む。

/// 点自由度の添字。並びは`拘束グラフ`の点一覧の順である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 点添字(u32);

impl 点添字 {
    pub fn 生成する(値: u32) -> Self {
        Self(値)
    }

    /// GPUのバイト列へ書く境界向けの生値。
    pub fn 値(&self) -> u32 {
        self.0
    }

    pub fn 配列添字(&self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("点添字がusizeに収まらない: {}", self.0))
    }
}
