//! 描画対象ごとの静的ユニフォーム。フレーム共通値と分け、対象ごとの重複を避ける。
//! 注意: バッファはストレージバッファとしても束縛する。個体が1体だけの対象は専用の個体変換バッファを確保せず、
//! このバッファの先頭112バイトを個体変換1件として読むためである(参照: `vulkan::instance_transform`)。

mod bytes;
#[cfg(test)]
mod bytes_tests;
mod content;

use ash::vk;
use blitz_math::{ローカル, ワールド, 変換};

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vulkan::host_buffer;
use crate::vulkan::instance_transform::content::個体変換内容;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct 描画対象ユニフォーム {
    pub(crate) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 描画対象ユニフォーム {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        ローカルからワールド: 変換<ローカル, ワールド>,
        マテリアル: &マテリアル素材,
    ) -> Result<Self, レンダラーエラー> {
        let 内容 = content::描画対象ユニフォーム内容 {
            変換: 個体変換内容::変換から作る(ローカルからワールド)?,
            ベースカラー係数: マテリアル.ベースカラー係数(),
            金属粗さ係数: [マテリアル.金属度係数(), マテリアル.粗さ係数()],
        };
        let バイト列 = bytes::バイト列にする(&内容);
        let (buffer, memory) = host_buffer::確保して書き込む(
            device,
            メモリプロパティ,
            &バイト列,
            vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        Ok(Self { buffer, memory })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferとmemoryはSelfが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}
