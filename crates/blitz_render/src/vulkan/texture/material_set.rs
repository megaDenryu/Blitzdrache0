//! マテリアル1つぶんの3テクスチャ(ベースカラー・金属粗さ・法線マップ)を
//! まとめて生成・破棄する(判断23)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

use super::テクスチャ;

pub(crate) struct マテリアルテクスチャ一式 {
    pub(crate) ベースカラー: テクスチャ,
    pub(crate) 金属粗さ: テクスチャ,
    pub(crate) 法線マップ: テクスチャ,
}

impl マテリアルテクスチャ一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        問い合わせ: 物理デバイス問い合わせ<'_>,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        マテリアル: &マテリアル素材,
    ) -> Result<Self, レンダラーエラー> {
        let ベースカラー = テクスチャ::生成する(device, 問い合わせ, メモリプロパティ, 転送環境, マテリアル.ベースカラー())?;

        let 金属粗さ = match テクスチャ::生成する(device, 問い合わせ, メモリプロパティ, 転送環境, マテリアル.金属粗さ())
        {
            Ok(テクスチャ) => テクスチャ,
            Err(誤り) => {
                ベースカラー.破棄する(device);
                return Err(誤り);
            }
        };

        let 法線マップ = match テクスチャ::生成する(device, 問い合わせ, メモリプロパティ, 転送環境, マテリアル.法線マップ())
        {
            Ok(テクスチャ) => テクスチャ,
            Err(誤り) => {
                ベースカラー.破棄する(device);
                金属粗さ.破棄する(device);
                return Err(誤り);
            }
        };

        Ok(Self {
            ベースカラー,
            金属粗さ,
            法線マップ,
        })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.ベースカラー.破棄する(device);
        self.金属粗さ.破棄する(device);
        self.法線マップ.破棄する(device);
    }
}
