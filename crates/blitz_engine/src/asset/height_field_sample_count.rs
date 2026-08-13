//! 高さ場の容量を標本数で表す型。格納バイト数への換算はこの型だけが知る。

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct 高さ場の標本数(u64);

impl 高さ場の標本数 {
    pub const fn 生成する(標本数: u64) -> Self {
        Self(標本数)
    }

    pub const fn 値(self) -> u64 {
        self.0
    }

    /// 現行の高さ場形式は標本1つをf32で格納する。形式が変わればこの換算も同じ版の変更として扱う。
    pub const fn 格納バイト数を求める(self) -> Option<u64> {
        const F32のバイト数: u64 = 4;
        self.0.checked_mul(F32のバイト数)
    }
}
