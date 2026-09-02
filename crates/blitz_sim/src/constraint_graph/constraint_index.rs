//! 拘束グラフの距離拘束を指す添字。点の添字と別の型にするのは、隣接表が両方を並べるため取り違えを型で止めるためである。

/// 距離拘束の添字。並びは`拘束グラフ`の拘束一覧の順である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 拘束添字(u32);

impl 拘束添字 {
    pub fn 生成する(値: u32) -> Self {
        Self(値)
    }

    /// GPUのバイト列へ書く境界向けの生値。
    pub fn 値(&self) -> u32 {
        self.0
    }

    pub fn 配列添字(&self) -> usize {
        usize::try_from(self.0).unwrap_or_else(|_| panic!("拘束添字がusizeに収まらない: {}", self.0))
    }

    /// 配列の添字から作る。拘束の数がu32に収まることは`拘束グラフ`の生成が保証する。
    pub(super) fn 配列添字から生成する(添字: usize) -> Self {
        Self(u32::try_from(添字).unwrap_or_else(|_| panic!("拘束の数がu32に収まらない: {添字}")))
    }
}
