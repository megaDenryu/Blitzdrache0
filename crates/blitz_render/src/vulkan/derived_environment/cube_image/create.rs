//! 派生の立方体画像の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけである。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::vulkan_object::{段の配列ビューを作る, 画像を作る, 立方体ビューを作る};
use super::派生の立方体画像;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    最詳細段の一辺: u32,
    段数: u32,
) -> Result<派生の立方体画像, レンダラーエラー> {
    let 画像 = 画像を作る(device, 最詳細段の一辺, 段数)?;
    let memory = match メモリを確保して結びつける(device, メモリプロパティ, 画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 全ビューを作る(device, 画像, 段数) {
        Ok((段ごとの配列ビュー, 立方体ビュー)) => Ok(派生の立方体画像 {
            画像,
            段ごとの配列ビュー,
            立方体ビュー,
            最詳細段の一辺,
            段数,
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

/// 段ごとの配列ビューと全段の立方体ビューを順に作る。途中で失敗したら、それまでに作ったビューを片付ける。
fn 全ビューを作る(
    device: &ash::Device, 画像: vk::Image, 段数: u32
) -> Result<(Vec<vk::ImageView>, vk::ImageView), レンダラーエラー> {
    let 段数の容量 = usize::try_from(段数).unwrap_or_else(|_| panic!("縮小段の数{段数}がusizeに収まらない"));
    let mut 段ごと = Vec::with_capacity(段数の容量);
    for 段 in 0..段数 {
        match 段の配列ビューを作る(device, 画像, 段) {
            Ok(ビュー) => 段ごと.push(ビュー),
            Err(誤り) => return Err(ビュー一覧を片付けて返す(device, &段ごと, 誤り)),
        }
    }
    match 立方体ビューを作る(device, 画像, 段数) {
        Ok(立方体ビュー) => Ok((段ごと, 立方体ビュー)),
        Err(誤り) => Err(ビュー一覧を片付けて返す(device, &段ごと, 誤り)),
    }
}

fn ビュー一覧を片付けて返す(
    device: &ash::Device, 一覧: &[vk::ImageView], 誤り: レンダラーエラー
) -> レンダラーエラー {
    for ビュー in 一覧 {
        // 安全性: ビューはこのスコープの唯一の所有者で、以降使用しない。
        unsafe { device.destroy_image_view(*ビュー, None) };
    }
    誤り
}
