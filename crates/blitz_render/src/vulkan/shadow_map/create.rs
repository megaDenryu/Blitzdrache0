//! シャドウマップの画像・メモリ・画像ビュー・比較サンプラーの確保。

mod image;

use ash::vk;

use self::image::{メモリを確保して結びつける, 画像を作る, 画像ビューを作る};
use super::sampler::比較サンプラーを作る;
use super::シャドウマップ;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
) -> Result<シャドウマップ, レンダラーエラー> {
    let 画像 = 画像を作る(device)?;
    let memory = match メモリを確保して結びつける(device, メモリプロパティ, 画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    let 画像ビュー = match 画像ビューを作る(device, 画像) {
        Ok(view) => view,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            return Err(誤り);
        }
    };
    let sampler = match 比較サンプラーを作る(device) {
        Ok(sampler) => sampler,
        Err(誤り) => {
            // 安全性: 画像と画像ビューはこのスコープの唯一の所有者で、以降使用しない。
            unsafe {
                device.destroy_image_view(画像ビュー, None);
                device.destroy_image(画像, None);
            }
            device.メモリを解放する(memory);
            return Err(誤り);
        }
    };
    Ok(シャドウマップ {
        画像,
        画像ビュー,
        sampler,
        memory,
    })
}
