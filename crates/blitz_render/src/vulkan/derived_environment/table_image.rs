//! 反射率積分表の2次元画像・メモリ・ビューの所有者。生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。
//!
//! 立方体の派生画像と別の型にするのは、この表が方向を持たない2次元の画像であり、層も縮小段も持たないためである。
//! 画素形式も2成分の半精度であり、4成分の立方体画像と別である。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

/// 反射率積分表の画素形式。第1成分が垂直入射の反射率に掛かる倍率、第2成分が反射率に依らず足す量である。
pub(in crate::vulkan) const 反射率積分表の画像形式: vk::Format = vk::Format::R16G16_SFLOAT;

pub(in crate::vulkan) struct 反射率積分表の画像 {
    pub(in crate::vulkan) 画像: vk::Image,
    pub(in crate::vulkan) ビュー: vk::ImageView,
    横: u32,
    縦: u32,
    memory: vk::DeviceMemory,
}

impl 反射率積分表の画像 {
    pub(in crate::vulkan) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        横: u32,
        縦: u32,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 横, 縦)
    }

    pub(in crate::vulkan) fn 範囲(&self) -> vk::Extent3D {
        vk::Extent3D {
            width: self.横,
            height: self.縦,
            depth: 1,
        }
    }

    pub(in crate::vulkan) fn グラフへ渡す寸法(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.横,
            height: self.縦,
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この画像は派生表現一式の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・ビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_image_view(self.ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
