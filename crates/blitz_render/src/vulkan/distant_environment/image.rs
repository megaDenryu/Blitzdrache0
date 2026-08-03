//! 遠方環境の立方体画像・メモリ・2つのビューの所有者。生成は起動時と検査の組み立て時の1回だけであり、
//! 以降のフレームは中身を焼き直すだけで作り直さない。生成の手順は`create`が担い、ここは保持と破棄だけを持つ。
//!
//! ビューを2つ持つのは、書く側と読む側で必要な種別が違うためである。生成のコンピュートは層番号を奥行きに取る
//! 2次元配列として書き、消費側は向きで参照できる立方体として読む。Vulkanは同じ画像から両方のビューを作れるため、
//! 画像そのものは1つで足りる。
//!
//! 用途にSTORAGEとSAMPLEDとTRANSFER_SRCとTRANSFER_DSTを立てるのは、コンピュートが書き、後段のパスがサンプラーで読み、
//! 検査が読み戻し、派生表現の検査が与えた中身をホストから書き込むためである。画素形式は大気のベイク済み画像と同じ
//! R16G16B16A16_SFLOATであり、ストレージ書き込みはVulkan仕様の必須対応形式のため実行時の対応確認は行わない。

mod create;
mod vulkan_object;

use ash::vk;

use crate::atmosphere::立方体の面数;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan) struct 遠方環境の立方体画像 {
    pub(in crate::vulkan) 画像: vk::Image,
    /// 生成のコンピュートが書き込む先。層番号を奥行きに取る2次元配列である。
    pub(in crate::vulkan) 配列ビュー: vk::ImageView,
    /// 消費側が向きで参照する先。立方体として標本する。
    pub(in crate::vulkan) 立方体ビュー: vk::ImageView,
    pub(in crate::vulkan) 面の一辺: u32,
    memory: vk::DeviceMemory,
}

impl 遠方環境の立方体画像 {
    pub(in crate::vulkan) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        面の一辺: u32,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 面の一辺)
    }

    /// 全6層を1度に走査する計算の発行の大きさ。奥行きが面の層番号である。
    pub(in crate::vulkan) fn 範囲(&self) -> vk::Extent3D {
        vk::Extent3D {
            width: self.面の一辺,
            height: self.面の一辺,
            depth: 1,
        }
    }

    /// グラフへ渡す寸法。グラフはこの値を描画領域にしか使わないため、コンピュートとコピーだけが触れる
    /// この画像では読まれない。
    pub(in crate::vulkan) fn グラフへ渡す寸法(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.面の一辺,
            height: self.面の一辺,
        }
    }

    pub(in crate::vulkan) fn 層数(&self) -> u32 {
        立方体の面数
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この画像は遠方環境一式の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・2つのビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_image_view(self.立方体ビュー, None);
            device.destroy_image_view(self.配列ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
