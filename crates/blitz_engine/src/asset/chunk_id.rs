//! チャンクID: 描画対象を所有するストリーミング単位の識別子。

/// シーン内で一意になる所有チャンクの番号。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct チャンクID(u64);

impl チャンクID {
    /// すべてのu64を有効な識別子として受け入れる。
    pub fn 生成する(番号: u64) -> Self {
        Self(番号)
    }

    /// 永続化・ログ境界で使う番号を返す。
    pub fn 番号を返す(self) -> u64 {
        self.0
    }
}
