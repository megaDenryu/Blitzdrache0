//! 遠方環境の立方体画像の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけである。途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::vulkan_object::{ビューを作る, 画像を作る};
use super::遠方環境の立方体画像;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    面の一辺: u32,
) -> Result<遠方環境の立方体画像, レンダラーエラー> {
    let 画像 = 画像を作る(device, 面の一辺)?;
    let memory = match メモリを確保して結びつける(device, メモリプロパティ, 画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 両方のビューを作る(device, 画像) {
        Ok([配列ビュー, 立方体ビュー]) => Ok(遠方環境の立方体画像 {
            画像,
            配列ビュー,
            立方体ビュー,
            面の一辺,
            memory,
        }),
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            Err(誤り)
        }
    }
}

fn メモリを確保して結びつける(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    画像: vk::Image,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: 画像は直前に生成済み。
    let 要件 = unsafe { device.get_image_memory_requirements(画像) };
    let メモリ型添字 = memory::デバイスローカルメモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let memory = memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::描画画像)?;
    // 安全性: 画像・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_image_memory(画像, memory, 0) } {
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    Ok(memory)
}

/// 書く側の2次元配列ビューと読む側の立方体ビューを順に作る。後者で失敗したら前者をその場で片付ける。
fn 両方のビューを作る(device: &ash::Device, 画像: vk::Image) -> Result<[vk::ImageView; 2], レンダラーエラー> {
    let 配列ビュー = ビューを作る(device, 画像, vk::ImageViewType::TYPE_2D_ARRAY)?;
    match ビューを作る(device, 画像, vk::ImageViewType::CUBE) {
        Ok(立方体ビュー) => Ok([配列ビュー, 立方体ビュー]),
        Err(誤り) => {
            // 安全性: 配列ビューはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image_view(配列ビュー, None) };
            Err(誤り)
        }
    }
}
