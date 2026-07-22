//! glTFメッシュ用の頂点・インデックスバッファ。ステージング+デバイスローカル転送で
//! 確保する(判断20)。アセットホットリロード(`シーンを差し替える`)のたびに破棄・再生成する。

pub(crate) mod bytes;
pub(crate) mod upload;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vertex::頂点;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct ジオメトリバッファ {
    pub(crate) 頂点バッファ: vk::Buffer,
    頂点メモリ: vk::DeviceMemory,
    pub(crate) インデックスバッファ: vk::Buffer,
    インデックスメモリ: vk::DeviceMemory,
    pub(crate) インデックス数: u32,
}

impl ジオメトリバッファ {
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let 頂点バイト列 = bytes::頂点をバイト列にする(頂点一覧);
        let (頂点バッファ, 頂点メモリ) = upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &頂点バイト列,
            vk::BufferUsageFlags::VERTEX_BUFFER,
        )?;

        let インデックスバイト列 = bytes::インデックスをバイト列にする(インデックス一覧);
        let (インデックスバッファ, インデックスメモリ) = match upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &インデックスバイト列,
            vk::BufferUsageFlags::INDEX_BUFFER,
        ) {
            Ok(結果) => 結果,
            Err(誤り) => {
                // 安全性: 頂点バッファ・頂点メモリはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    device.destroy_buffer(頂点バッファ, None);
                    device.free_memory(頂点メモリ, None);
                }
                return Err(誤り);
            }
        };

        let インデックス数 =
            u32::try_from(インデックス一覧.len()).unwrap_or_else(|_| panic!("インデックス数がu32に収まらない: {}", インデックス一覧.len()));

        Ok(Self {
            頂点バッファ,
            頂点メモリ,
            インデックスバッファ,
            インデックスメモリ,
            インデックス数,
        })
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_buffer(self.頂点バッファ, None);
            device.free_memory(self.頂点メモリ, None);
            device.destroy_buffer(self.インデックスバッファ, None);
            device.free_memory(self.インデックスメモリ, None);
        }
    }
}
