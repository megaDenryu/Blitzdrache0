use ash::vk::{self, Handle};

use super::メモリ台帳;
use crate::gpu_memory_stats::GPUメモリ用途;

#[test]
fn 確保と解放を用途別に集計する() {
    let 台帳 = メモリ台帳::生成する(4_096);
    let first = vk::DeviceMemory::from_raw(1);
    let second = vk::DeviceMemory::from_raw(2);

    台帳.確保を記録する(first, 64, GPUメモリ用途::デバイスバッファ);
    台帳.確保を記録する(second, 128, GPUメモリ用途::描画画像);
    台帳.解放を記録する(first);

    let 統計 = 台帳.統計を取得する();
    assert_eq!(統計.現在確保数(), 1);
    assert_eq!(統計.最大同時確保数(), 2);
    assert_eq!(統計.デバイス上限(), 4_096);
    assert_eq!(統計.用途別確保量()[0].バイト数(), 0);
    assert_eq!(統計.用途別確保量()[2].バイト数(), 128);
}
