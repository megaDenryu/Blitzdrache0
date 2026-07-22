//! vkCmdCopyImageToBufferの記録。読み戻し要求時のみ呼ばれる。

use ash::vk;

pub(super) fn コピーを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    バッファ: vk::Buffer,
    寸法: vk::Extent2D,
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
            width: 寸法.width,
            height: 寸法.height,
            depth: 1,
        });
    let 領域一覧 = [領域];
    // 安全性: command_bufferは記録中、画像はTRANSFER_SRC_OPTIMALへ遷移済み、
    // バッファは呼び出し元が寸法ぶんの容量で確保済み(読み戻しバッファの確保)。
    unsafe {
        device.cmd_copy_image_to_buffer(command_buffer, 画像, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, バッファ, &領域一覧);
    }
}
