//! Vulkan専用メモリ確保の実行時統計。ashの型を公開せず、用途別の現在量を上位層へ渡す。
//! 用途の分類そのものは`usage`が持つ。

mod usage;

pub use usage::GPUメモリ用途;

/// 1用途の現在確保量。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GPUメモリ用途別確保量 {
    用途: GPUメモリ用途,
    バイト数: u64,
}

impl GPUメモリ用途別確保量 {
    pub(crate) fn 生成する(用途: GPUメモリ用途, バイト数: u64) -> Self {
        Self { 用途, バイト数 }
    }

    pub fn 用途(&self) -> GPUメモリ用途 {
        self.用途
    }

    pub fn バイト数(&self) -> u64 {
        self.バイト数
    }
}

/// 1レンダラーが所有するVulkan専用メモリ確保のスナップショット。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GPUメモリ統計 {
    現在確保数: usize,
    最大同時確保数: usize,
    デバイス上限: u32,
    用途別確保量: [GPUメモリ用途別確保量; 6],
}

impl GPUメモリ統計 {
    pub(crate) fn 生成する(現在確保数: usize, 最大同時確保数: usize, デバイス上限: u32, 用途別確保量: [u64; 6]) -> Self {
        Self {
            現在確保数,
            最大同時確保数,
            デバイス上限,
            用途別確保量: GPUメモリ用途::一覧.map(|用途| GPUメモリ用途別確保量::生成する(用途, 用途別確保量[用途.添字()])),
        }
    }

    pub fn 現在確保数(&self) -> usize {
        self.現在確保数
    }

    pub fn 最大同時確保数(&self) -> usize {
        self.最大同時確保数
    }

    pub fn デバイス上限(&self) -> u32 {
        self.デバイス上限
    }

    pub fn 用途別確保量(&self) -> &[GPUメモリ用途別確保量; 6] {
        &self.用途別確保量
    }
}
