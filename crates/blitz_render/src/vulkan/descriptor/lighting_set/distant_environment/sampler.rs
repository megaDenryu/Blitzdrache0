//! 遠方環境の3つの画像を読む固定サンプラーを所有する資源型。触れるのはサンプラー1つだけであり、
//! どの束縛番号へ結ぶかは知らない。
//!
//! 段の間を補間するのは、粗さが段の間の値を取るためである。縁を伸ばすのは、反射率積分表の端
//! (粗さ0と1、視線が面と平行)で表の外を読ませないためである。
//!
//! 注意: `Drop`を持たない。破棄の順番は照明問い合わせ資源束が決める。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 遠方環境を読むサンプラー(vk::Sampler);

impl 遠方環境を読むサンプラー {
    pub(crate) fn 確保する(確保係: &GPU資源の確保係<'_>) -> Result<Self, レンダラーエラー> {
        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .min_lod(0.0)
            .max_lod(vk::LOD_CLAMP_NONE)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false);
        Ok(Self(確保係.標本の取り方からサンプラーを確保する(&create_info)?))
    }

    /// 書き込み記述子へ渡す境界。
    pub(super) fn ハンドル(&self) -> vk::Sampler {
        self.0
    }

    /// 前提: 呼び出し元はGPU側の使用完了を保証する。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: サンプラーはSelfが唯一の所有者である。
        unsafe { device.destroy_sampler(self.0, None) };
    }
}
