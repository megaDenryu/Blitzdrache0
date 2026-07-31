//! glTFメッシュ用の頂点・インデックスバッファ。ステージング+デバイスローカル転送で
//! 確保する(判断20)。アセットホットリロード(`シーンを差し替える`)のたびに破棄・再生成する。
//! インデックスの総数を持たないのは、描画発行がプリミティブごとに自分のインデックス区間を持つためである
//! (参照: `_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」)。

pub(crate) mod bytes;
pub(crate) mod upload;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vertex::頂点;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct ジオメトリバッファ {
    pub(crate) 頂点バッファ: vk::Buffer,
    頂点メモリ: vk::DeviceMemory,
    pub(crate) インデックスバッファ: vk::Buffer,
    インデックスメモリ: vk::DeviceMemory,
}

impl ジオメトリバッファ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
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
                // 安全性: 頂点バッファはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(頂点バッファ, None) };
                device.メモリを解放する(頂点メモリ);
                return Err(誤り);
            }
        };

        Ok(Self {
            頂点バッファ,
            頂点メモリ,
            インデックスバッファ,
            インデックスメモリ,
        })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各バッファはSelfが唯一の所有者で、GPU側の使用は完了済み。
        unsafe {
            device.destroy_buffer(self.頂点バッファ, None);
            device.destroy_buffer(self.インデックスバッファ, None);
        }
        device.メモリを解放する(self.頂点メモリ);
        device.メモリを解放する(self.インデックスメモリ);
    }
}
