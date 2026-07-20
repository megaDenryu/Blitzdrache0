//! バッファ仮想リソースの世代付きハンドル。画像ハンドルと対になる型。
//!
//! 参照: `_doc/設計/レンダーグラフ.md`。波1ではバッファを使うパスが無いため未使用だが、
//! 波2のGPU粒子トイ（コンピュート読み書き）で使う型をここで先に用意する。
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct バッファハンドル {
    添字: u32,
    世代: u32,
}

#[allow(dead_code)]
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
