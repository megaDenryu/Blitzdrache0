//! 描画対象ID: 1つのシーン内で描画対象を区別する識別子。

/// シーン内で一意になる描画対象の番号。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct 描画対象ID(u64);

impl 描画対象ID {
    /// すべてのu64を有効な識別子として受け入れる。
    pub fn 生成する(番号: u64) -> Self {
        Self(番号)
    }

    /// 永続化・ログ境界で使う番号を返す。
    pub fn 番号を返す(self) -> u64 {
        self.0
    }
}
