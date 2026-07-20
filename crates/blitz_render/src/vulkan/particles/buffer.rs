//! 粒子ストレージバッファ(デバイスローカル、位置vec4+速度vec4/粒子)。
//! 初回構築時に1回だけ確保し、以後は毎フレーム使い回す(判断29・レンダーグラフ.md
//! 「仮想リソース」のグラフ管理の項)。ステージング転送での初期化はジオメトリバッファと
//! 同じ転送基盤(`vulkan::geometry::upload`)を再利用する。

use ash::vk;

use super::initial_data;
use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct 粒子バッファ {
    pub(crate) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 粒子バッファ {
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
    ) -> Result<Self, レンダラーエラー> {
        let 初期バイト列 = initial_data::初期バイト列を生成する();
        let (buffer, memory) = upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &初期バイト列,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        Ok(Self { buffer, memory })
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: buffer・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }
}
