//! ストリーミング対象が消費するRAM・VRAMのバイト量。

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ストリーミングメモリ量 {
    pub ramバイト数: u64,
    pub vramバイト数: u64,
}

impl ストリーミングメモリ量 {
    pub fn 生成する(ramバイト数: u64, vramバイト数: u64) -> Self {
        Self {
            ramバイト数, vramバイト数
        }
    }

    pub(super) fn 加算する(self, 追加: Self) -> Option<Self> {
        Some(Self {
            ramバイト数: self.ramバイト数.checked_add(追加.ramバイト数)?,
            vramバイト数: self.vramバイト数.checked_add(追加.vramバイト数)?,
        })
    }

    /// RAMとVRAMのどちらかが上限を超えているか。上限との比較規則をこの1箇所だけが定める。
    pub(super) fn いずれかが上限を超えるか(self, 上限: Self) -> bool {
        self.ramバイト数 > 上限.ramバイト数 || self.vramバイト数 > 上限.vramバイト数
    }
}
