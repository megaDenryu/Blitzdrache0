//! バッファ仮想リソースの世代付きハンドル。画像ハンドルと対になる型。
//! 実体(vk::Buffer)はグラフ実行器のバッファレジストリだけが知り、宣言側は
//! 添字と世代の組でのみ参照する。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct バッファハンドル {
    添字: u32,
    世代: u32,
}

impl バッファハンドル {
    pub(crate) fn 生成する(添字: u32, 世代: u32) -> Self {
        Self { 添字, 世代 }
    }

    pub(crate) fn 添字(&self) -> u32 {
        self.添字
    }

    pub(crate) fn 世代(&self) -> u32 {
        self.世代
    }
}
