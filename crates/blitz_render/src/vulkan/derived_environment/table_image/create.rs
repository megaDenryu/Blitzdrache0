//! 反射率積分表の画像の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけであり、
//! 表は大気にも時刻にも依らないため以降1度も焼き直さない。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::{反射率積分表の画像, 反射率積分表の画像形式};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 反射率積分表の画像を生成する(
    確保係: &GPU資源の確保係<'_>,
    横: u32,
    縦: u32,
) -> Result<反射率積分表の画像, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 画像を作る(確保係, 横, 縦)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match ビューを作る(確保係, 画像) {
        Ok(ビュー) => Ok(反射率積分表の画像 {
            画像,
            ビュー,
            横,
            縦,
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

/// 用途にSTORAGEとSAMPLEDとTRANSFER_SRCとTRANSFER_DSTを立てるのは、コンピュートが書き、標準PBRがサンプラーで読み、
/// 検査が読み戻し、検収が解析入力を転送で書き込むためである。
fn 画像を作る(確保係: &GPU資源の確保係<'_>, 横: u32, 縦: u32) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(反射率積分表の画像形式)
        .extent(vk::Extent3D {
            width: 横,
            height: 縦,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    確保係.画像の作り方から画像を確保する(&create_info)
}

fn ビューを作る(確保係: &GPU資源の確保係<'_>, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(反射率積分表の画像形式)
        .subresource_range(部分範囲);
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}
