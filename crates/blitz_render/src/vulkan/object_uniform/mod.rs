//! 描画対象ごとの静的ユニフォーム。フレーム共通値と分け、対象ごとの重複を避ける。

mod bytes;
#[cfg(test)]
mod bytes_tests;
mod content;

use ash::vk;
use blitz_math::{ローカル, ワールド, 変換};
use glam::{Mat3, Mat4};

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vulkan::host_buffer;
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
        let 内容 = 内容を作る(ローカルからワールド, マテリアル)?;
        let バイト列 = bytes::バイト列にする(&内容);
        let (buffer, memory) = host_buffer::確保して書き込む(device, メモリプロパティ, &バイト列, vk::BufferUsageFlags::UNIFORM_BUFFER)?;
        Ok(Self { buffer, memory })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferとmemoryはSelfが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}

fn 内容を作る(
    ローカルからワールド: 変換<ローカル, ワールド>,
    マテリアル: &マテリアル素材,
) -> Result<content::描画対象ユニフォーム内容, レンダラーエラー> {
    let 生行列 = ローカルからワールド.gpu境界用列優先配列();
    let 行列 = Mat4::from_cols_array_2d(&生行列);
    let 法線基底 = Mat3::from_mat4(行列);
    let 行列式 = 法線基底.determinant();
    if !行列式.is_finite() || 行列式.abs() <= f32::EPSILON {
        return Err(レンダラーエラー::描画対象変換非可逆);
    }
    Ok(content::描画対象ユニフォーム内容 {
        ローカルからワールド: 生行列,
        法線ローカルからワールド: 法線基底.inverse().transpose().to_cols_array_2d(),
        ベースカラー係数: マテリアル.ベースカラー係数(),
        金属粗さ係数: [マテリアル.金属度係数(), マテリアル.粗さ係数()],
    })
}
