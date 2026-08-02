//! 資源表世代の版番号。担当するのは、版が単調に進むことと、番号を使い切ったときに新しい世代を作れないことを型で表すことである。
//!
//! 番号を1から始めるのは、初期化していない値と最初の世代を取り違えないためである。
//! 番号の枯渇を型付きの失敗にするのは、折り返すと退役待ちの旧世代と同じ番号の新世代が生まれ、
//! 材質GPU参照の世代一致検査が通ってしまうためである。

use crate::error::材質資源表エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct 資源表世代ID {
    値: u32,
}

impl 資源表世代ID {
    pub(crate) const fn 最初() -> Self {
        Self { 値: 1 }
    }

    /// 番号の枯渇の検査だけが、末尾の番号から始める世代を組み立てるのに使う。本番は`最初`と`次を作る`だけを通る。
    #[cfg(test)]
    pub(in crate::vulkan::material_table) const fn 番号から生成する(値: u32) -> Self {
        Self { 値 }
    }

    pub(crate) fn 次を作る(self) -> Result<Self, 材質資源表エラー> {
        match self.値.checked_add(1) {
            Some(次) => Ok(Self { 値: 次 }),
            None => Err(材質資源表エラー::世代番号の枯渇),
        }
    }

    pub(crate) const fn 番号(self) -> u32 {
        self.値
    }
}
