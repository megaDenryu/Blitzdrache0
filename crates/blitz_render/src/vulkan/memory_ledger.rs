//! 1論理デバイス内のVulkan専用メモリ確保を追跡する台帳。

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use ash::vk::{self, Handle};

use crate::gpu_memory_stats::{GPUメモリ用途, GPUメモリ統計};

#[derive(Clone, Copy)]
struct 確保記録 {
    バイト数: u64,
    用途: GPUメモリ用途,
}

#[derive(Default)]
struct 台帳状態 {
    確保一覧: HashMap<u64, 確保記録>,
    最大同時確保数: usize,
    用途別確保量: [u64; 6],
}

pub(crate) struct メモリ台帳 {
    状態: Mutex<台帳状態>,
    デバイス上限: u32,
}

impl メモリ台帳 {
    pub(crate) fn 生成する(デバイス上限: u32) -> Self {
        Self {
            状態: Mutex::new(台帳状態::default()),
            デバイス上限,
        }
    }

    pub(crate) fn 確保を記録する(&self, memory: vk::DeviceMemory, バイト数: u64, 用途: GPUメモリ用途) {
        let mut 状態 = self.状態をロックする();
        if 状態.確保一覧.contains_key(&memory.as_raw()) {
            panic!("同じVulkanメモリハンドルが解放前に再登録された");
        }
        let Some(合計) = 状態.用途別確保量[用途.添字()].checked_add(バイト数) else {
            panic!("GPUメモリの用途別確保量がu64の上限を超えた");
        };
        状態.確保一覧.insert(memory.as_raw(), 確保記録 { バイト数, 用途 });
        状態.用途別確保量[用途.添字()] = 合計;
        状態.最大同時確保数 = 状態.最大同時確保数.max(状態.確保一覧.len());
    }

    pub(crate) fn 解放を記録する(&self, memory: vk::DeviceMemory) {
        let mut 状態 = self.状態をロックする();
        let Some(記録) = 状態.確保一覧.get(&memory.as_raw()).copied() else {
            panic!("台帳に存在しないVulkanメモリを解放しようとした");
        };
        let Some(残量) = 状態.用途別確保量[記録.用途.添字()].checked_sub(記録.バイト数) else {
            panic!("GPUメモリの用途別確保量が確保記録より小さい");
        };
        状態.確保一覧.remove(&memory.as_raw());
        状態.用途別確保量[記録.用途.添字()] = 残量;
    }

    pub(crate) fn 統計を取得する(&self) -> GPUメモリ統計 {
        let 状態 = self.状態をロックする();
        GPUメモリ統計::生成する(状態.確保一覧.len(), 状態.最大同時確保数, self.デバイス上限, 状態.用途別確保量)
    }

    fn 状態をロックする(&self) -> MutexGuard<'_, 台帳状態> {
        match self.状態.lock() {
            Ok(状態) => 状態,
            Err(_) => panic!("GPUメモリ台帳の排他制御が破損した"),
        }
    }
}

#[cfg(test)]
mod memory_ledger_tests;
