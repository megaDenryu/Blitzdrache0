//! Vulkan専用メモリ確保の実行時統計。ashの型を公開せず、用途別の現在量を上位層へ渡す。

/// GPUメモリ確保を、確保した資源の役割で分類する。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPUメモリ用途 {
    デバイスバッファ,
    ホストバッファ,
    描画画像,
    テクスチャ画像,
    読み戻しバッファ,
}

impl GPUメモリ用途 {
    pub(crate) const 一覧: [Self; 5] = [
        Self::デバイスバッファ,
        Self::ホストバッファ,
        Self::描画画像,
        Self::テクスチャ画像,
        Self::読み戻しバッファ,
    ];

    pub fn 名称(self) -> &'static str {
        match self {
            Self::デバイスバッファ => "デバイスバッファ",
            Self::ホストバッファ => "ホストバッファ",
            Self::描画画像 => "描画画像",
            Self::テクスチャ画像 => "テクスチャ画像",
            Self::読み戻しバッファ => "読み戻しバッファ",
        }
    }

    pub(crate) fn 添字(self) -> usize {
        match self {
            Self::デバイスバッファ => 0,
            Self::ホストバッファ => 1,
            Self::描画画像 => 2,
            Self::テクスチャ画像 => 3,
            Self::読み戻しバッファ => 4,
        }
    }
}

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
    用途別確保量: [GPUメモリ用途別確保量; 5],
}

impl GPUメモリ統計 {
    pub(crate) fn 生成する(現在確保数: usize, 最大同時確保数: usize, デバイス上限: u32, 用途別確保量: [u64; 5]) -> Self {
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

    pub fn 用途別確保量(&self) -> &[GPUメモリ用途別確保量; 5] {
        &self.用途別確保量
    }
}
