//! 遠方環境の立方体画像の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけである。途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::vulkan_object::{ビューを作る, 画像を作る};
use super::遠方環境の立方体画像;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>, 面の一辺: u32
) -> Result<遠方環境の立方体画像, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 画像を作る(確保係, 面の一辺)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 両方のビューを作る(確保係, 画像) {
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

/// 書く側の2次元配列ビューと読む側の立方体ビューを順に作る。後者で失敗したら前者をその場で片付ける。
fn 両方のビューを作る(確保係: &GPU資源の確保係<'_>, 画像: vk::Image) -> Result<[vk::ImageView; 2], レンダラーエラー> {
    let 配列ビュー = ビューを作る(確保係, 画像, vk::ImageViewType::TYPE_2D_ARRAY)?;
    match ビューを作る(確保係, 画像, vk::ImageViewType::CUBE) {
        Ok(立方体ビュー) => Ok([配列ビュー, 立方体ビュー]),
        Err(誤り) => {
            // 安全性: 配列ビューはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { 確保係.論理デバイス().destroy_image_view(配列ビュー, None) };
            Err(誤り)
        }
    }
}
