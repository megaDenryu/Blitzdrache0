//! 大気LUT1枚ぶんの画像・メモリ・ビューの所有者。生成は起動時の1回だけであり、以降のフレームは中身を焼き直すだけで作り直さない。
//! 生成の手順は`create`が担い、ここは保持と破棄だけを持つ(`hdr_target`と同じ様式)。
//! 平面と立体の違いは`大気LUTの形`が持ち、この型は形をそのまま運ぶ。
//!
//! 用途にSTORAGEとSAMPLEDとTRANSFER_SRCを立てるのは、コンピュートが書き、後段のパスがサンプラーで読み、
//! 検査が読み戻すためである。R16G16B16A16_SFLOATのストレージ書き込みはVulkan仕様の必須対応形式のため、
//! 実行時の対応確認は行わない(HDR中間画像と同じ扱い)。

mod create;

use ash::vk;

use super::{大気LUTの形, 大気LUT画像入力};
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const 大気LUT形式: vk::Format = vk::Format::R16G16B16A16_SFLOAT;

pub(crate) struct 大気LUT画像 {
    pub(crate) 画像: vk::Image,
    pub(crate) 画像ビュー: vk::ImageView,
    pub(crate) 形: 大気LUTの形,
    memory: vk::DeviceMemory,
}

impl 大気LUT画像 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        形: 大気LUTの形,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 形)
    }

    /// グラフへ資源を登録するのに要る3つだけを取り出す。メモリのハンドルは外へ出さない。
    pub(crate) fn 画像入力を作る(&self) -> 大気LUT画像入力 {
        大気LUT画像入力 {
            画像: self.画像,
            ビュー: self.画像ビュー,
            形: self.形,
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この画像は大気LUT一式の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・画像ビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_image_view(self.画像ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
