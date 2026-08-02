//! 資源表世代のテクスチャをVulkanの画像として常駐させる本番の供給元。担当するのは、既存のテクスチャ生成経路を
//! 世代の構築と退役から呼べる1つの口へ束ねることである。
//!
//! 前提: 借りている論理デバイス・物理デバイスへの問い合わせ・転送実行環境は、この供給元より長生きする
//! (レンダラーの破棄順は`renderer/destroy.rs`が持ち、転送実行環境の破棄はこの供給元を使い終えた後である)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::texture::テクスチャ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

use super::supplier::常駐テクスチャ供給元;

pub(crate) struct デバイス常駐供給元<'環境> {
    device: &'環境 GPUデバイス,
    問い合わせ: 物理デバイス問い合わせ<'環境>,
    メモリプロパティ: vk::PhysicalDeviceMemoryProperties,
    転送環境: &'環境 転送実行環境,
}

impl<'環境> デバイス常駐供給元<'環境> {
    pub(crate) fn 生成する(
        device: &'環境 GPUデバイス,
        問い合わせ: 物理デバイス問い合わせ<'環境>,
        メモリプロパティ: vk::PhysicalDeviceMemoryProperties,
        転送環境: &'環境 転送実行環境,
    ) -> Self {
        Self {
            device,
            問い合わせ,
            メモリプロパティ,
            転送環境,
        }
    }
}

impl 常駐テクスチャ供給元 for デバイス常駐供給元<'_> {
    type 常駐画像 = テクスチャ;

    fn 常駐させる(&mut self, 素材: &テクスチャ素材) -> Result<テクスチャ, レンダラーエラー> {
        テクスチャ::生成する(self.device, self.問い合わせ, &self.メモリプロパティ, self.転送環境, 素材)
    }

    fn 退役させる(&mut self, 画像: テクスチャ) {
        画像.破棄する(self.device);
    }
}
