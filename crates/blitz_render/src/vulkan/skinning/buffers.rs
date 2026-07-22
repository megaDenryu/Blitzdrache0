//! スキニングの4種バッファ(判断44): レスト頂点(読み)・スキン属性(読み)・
//! スキン行列(フレームインフライト2重、ホスト可視で毎フレーム書く)・スキン済み頂点(書き)。
//! 生成手順は`create`にある。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct スキニングバッファ {
    レスト頂点: (vk::Buffer, vk::DeviceMemory),
    属性: (vk::Buffer, vk::DeviceMemory),
    行列一覧: [(vk::Buffer, vk::DeviceMemory); フレームインフライト数],
    pub(super) 出力: (vk::Buffer, vk::DeviceMemory),
}

pub(super) use create::生成する;

impl スキニングバッファ {
    pub(super) fn レスト頂点buffer(&self) -> vk::Buffer {
        self.レスト頂点.0
    }

    pub(super) fn 属性buffer(&self) -> vk::Buffer {
        self.属性.0
    }

    pub(super) fn 行列buffer(&self, フレーム添字: usize) -> vk::Buffer {
        self.行列一覧[フレーム添字].0
    }

    /// 前提: 呼び出しはフェンス待ち後(このスロットの前回GPU使用の完了後。判断24と同じ規律)。
    pub(super) fn 行列を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: usize,
        行列一覧: &[[f32; 16]],
    ) -> Result<(), レンダラーエラー> {
        let mut バイト列 = Vec::with_capacity(行列一覧.len() * 64);
        for 行列 in 行列一覧 {
            for 成分 in 行列 {
                バイト列.extend_from_slice(&成分.to_le_bytes());
            }
        }
        host_buffer::上書きする(device, self.行列一覧[フレーム添字].1, &バイト列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for &(buffer, memory) in [self.レスト頂点, self.属性, self.出力].iter().chain(self.行列一覧.iter()) {
            // 安全性: 各バッファはSelfが唯一の所有者で、GPU側の使用は完了済み。
            unsafe { device.destroy_buffer(buffer, None) };
            device.メモリを解放する(memory);
        }
    }
}
