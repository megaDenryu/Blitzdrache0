//! ステージングバッファ→mip0コピー→縮小段チェーン生成を1回の一時コマンドバッファで行う。
//! レイアウト遷移バリアの組み立ては`barrier`に委ねる。

mod barrier;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

use super::mip_chain;

#[allow(clippy::too_many_arguments)]
pub(super) fn ホストの画素列を画像へ転送する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    image: vk::Image,
    幅: u32,
    高さ: u32,
    mip数: u32,
    rgba8: &[u8],
) -> Result<(), レンダラーエラー> {
    let (ステージングバッファ, ステージングメモリ) =
        host_buffer::確保して書き込む(device, メモリプロパティ, rgba8, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let 実行結果 =
        ステージングバッファから縮小段チェーンまで転送する(転送環境, ステージングバッファ, image, 幅, 高さ, mip数);

    // 安全性: 転送実行は完了済みで、ステージングバッファは以降使用しない。
    unsafe { device.destroy_buffer(ステージングバッファ, None) };
    device.メモリを解放する(ステージングメモリ);
    実行結果
}

fn ステージングバッファから縮小段チェーンまで転送する(
    転送環境: &転送実行環境,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    幅: u32,
    高さ: u32,
    mip数: u32,
) -> Result<(), レンダラーエラー> {
    let 一時 = 転送環境.転送コマンドを積み始める()?;
    let device = 一時.論理デバイス();
    let command_buffer = 一時.積む先のコマンドバッファ();
    barrier::全レベルを転送先レイアウトへ遷移する(device, command_buffer, image, mip数);
    mip0へコピーする(device, command_buffer, ステージングバッファ, image, 幅, 高さ);
    mip_chain::縮小段チェーンを積む(device, command_buffer, image, 幅, 高さ, mip数);
    一時.送信して完了を待つ()
}

fn mip0へコピーする(
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
        .image_extent(vk::Extent3D {
            width: 幅,
            height: 高さ,
            depth: 1,
        });
    // 安全性: command_bufferは積み込み中。imageはTRANSFER_DST_OPTIMALへ遷移済み。
    // ステージングバッファは呼び出し元がrgba8と同じ長さで確保・書き込み済み。
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
