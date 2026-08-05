//! 局所可視度の2枚の画像(ぼかす前とぼかした後)。どちらも画面と同じ寸法で、8ビット無符号正規化の4成分形式である。
//!
//! 1成分の形式を選ばないのは、8ビット1成分への記憶画像の書き込みがVulkanの必須対応形式でないためである。
//! 4成分のR8G8B8A8_UNORMは必須対応であり、使うのは第1成分だけである。
//! 注意: 形式を変えるときは`shaders/local_visibility_occlusion.slang`と`local_visibility_blur.slang`の
//! `[format("rgba8")]`宣言も揃える。食い違うとパイプラインの生成が落ちる。
//!
//! 2枚とも常に画面寸法で確保するのは、局所可視性補正を持たない世界でも消費側が同じ画像を読むためである。
//! 1×1の代用画像にすると、画素の位置での読み出しが範囲外の0を返し、拡散間接が全画面で消える。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

/// 前提: R8G8B8A8_UNORMの記憶画像・サンプル・転送はVulkan仕様の必須対応のため、実行時の対応確認は行わない。
pub(crate) const 局所可視度の形式: vk::Format = vk::Format::R8G8B8A8_UNORM;

/// 画像1枚とその裏付け。
pub(crate) struct 局所可視度の画像 {
    pub(crate) 画像: vk::Image,
    pub(crate) 画像ビュー: vk::ImageView,
    memory: vk::DeviceMemory,
}

impl 局所可視度の画像 {
    fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・画像ビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_image_view(self.画像ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}

/// 2工程が読み書きする2枚の組。生の可視度は遮蔽の標本化が書いてぼかしが読み、ぼかし後はぼかしが書いて画素段が読む。
pub(crate) struct 局所可視度の画像組 {
    pub(crate) 生: 局所可視度の画像,
    pub(crate) ぼかし後: 局所可視度の画像,
}

impl 局所可視度の画像組 {
    /// ぼかし後の生成に失敗したら生の側を片付ける。
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        寸法: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        let 生 = create::生成する(device, メモリプロパティ, 寸法)?;
        match create::生成する(device, メモリプロパティ, 寸法) {
            Ok(ぼかし後) => Ok(Self { 生, ぼかし後 }),
            Err(誤り) => {
                生.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.ぼかし後.破棄する(device);
        self.生.破棄する(device);
    }
}
