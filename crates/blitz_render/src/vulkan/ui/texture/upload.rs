//! ステージングバッファ経由でUIテクスチャの唯一のミップレベルへ転送する
//! (マテリアルテクスチャと異なりミップチェーン生成は行わない)。レイアウト遷移
//! バリアの組み立ては`barrier`に委ねる。

mod barrier;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::transfer::転送実行環境;

pub(super) fn 記録して転送する(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    image: vk::Image,
    幅: u32,
    高さ: u32,
    rgba8: &[u8],
) -> Result<(), レンダラーエラー> {
    let (ステージングバッファ, ステージングメモリ) =
        host_buffer::確保して書き込む(device, メモリプロパティ, rgba8, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let 実行結果 = 転送環境.一括実行する(device, |command_buffer| {
        barrier::転送先へ遷移する(device, command_buffer, image);
        コピーする(device, command_buffer, ステージングバッファ, image, 幅, 高さ);
        barrier::シェーダー読み取り専用へ遷移する(device, command_buffer, image);
    });

    // 安全性: ステージングバッファ・メモリはこのスコープの唯一の所有者で、
    // 転送完了(一括実行するがfence待ち済み)後は不要。
    unsafe {
        device.destroy_buffer(ステージングバッファ, None);
        device.free_memory(ステージングメモリ, None);
    }
    実行結果
}

fn コピーする(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    幅: u32,
    高さ: u32,
) {
    let 領域 = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D { width: 幅, height: 高さ, depth: 1 });
    // 安全性: command_bufferは記録中。imageはTRANSFER_DST_OPTIMALへ遷移済み。
    unsafe {
        device.cmd_copy_buffer_to_image(
            command_buffer,
            ステージングバッファ,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[領域],
        );
    }
}
