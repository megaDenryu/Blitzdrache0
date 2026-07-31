//! ステージングバッファ経由でUIテクスチャの唯一の縮小段レベルへ転送する
//! (マテリアルテクスチャと異なり縮小段チェーン生成は行わない)。レイアウト遷移
//! バリアの組み立ては`barrier`に委ねる。

mod barrier;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(super) fn 記録して転送する(
    device: &GPUデバイス,
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

    // 安全性: 転送実行は完了済みで、ステージングバッファは以降使用しない。
    unsafe { device.destroy_buffer(ステージングバッファ, None) };
    device.メモリを解放する(ステージングメモリ);
    実行結果
}

fn コピーする(
    device: &ash::Device, command_buffer: vk::CommandBuffer, ステージングバッファ: vk::Buffer, image: vk::Image, 幅: u32, 高さ: u32
) {
    let 領域 = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: 幅,
            height: 高さ,
            depth: 1,
        });
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
