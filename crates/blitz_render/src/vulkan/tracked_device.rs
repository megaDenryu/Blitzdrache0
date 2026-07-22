//! ash論理デバイスと、そのデバイスに属する専用メモリ台帳を同じ所有境界に置く。

use std::ops::Deref;

use ash::vk;

use super::memory_ledger::メモリ台帳;
use crate::gpu_memory_stats::{GPUメモリ用途, GPUメモリ統計};

pub(crate) struct GPUデバイス {
    handle: ash::Device,
    メモリ台帳: メモリ台帳,
}

impl GPUデバイス {
    pub(crate) fn 生成する(handle: ash::Device, メモリ確保上限: u32) -> Self {
        Self {
            handle,
            メモリ台帳: メモリ台帳::生成する(メモリ確保上限),
        }
    }

    pub(crate) fn 確保を記録する(&self, memory: vk::DeviceMemory, バイト数: u64, 用途: GPUメモリ用途) {
        self.メモリ台帳.確保を記録する(memory, バイト数, 用途);
    }

    pub(crate) fn メモリを解放する(&self, memory: vk::DeviceMemory) {
        self.メモリ台帳.解放を記録する(memory);
        // 安全性: 台帳がmemoryをこのdeviceで確保した未解放ハンドルとして検証した。資源の破棄とGPU使用完了は各所有者が保証する。
        unsafe { self.handle.free_memory(memory, None) };
    }

    pub(crate) fn メモリ統計を取得する(&self) -> GPUメモリ統計 {
        self.メモリ台帳.統計を取得する()
    }

    pub(crate) fn 全メモリ解放を確認する(&self) {
        let 残数 = self.メモリ統計を取得する().現在確保数();
        if 残数 != 0 {
            panic!("論理デバイス破棄前にVulkan専用メモリが{残数}件残っている");
        }
    }
}

impl Deref for GPUデバイス {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
