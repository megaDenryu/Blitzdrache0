//! ステージングバッファ→mip0コピー→ミップチェーン生成を1回の一時コマンドバッファで行う。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

use super::mip_chain;

#[allow(clippy::too_many_arguments)]
pub(super) fn 記録して転送する(
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

    let 実行結果 = 転送環境.一括実行する(device, |command_buffer| {
        全レベルを転送先レイアウトへ遷移する(device, command_buffer, image, mip数);
        mip0へコピーする(device, command_buffer, ステージングバッファ, image, 幅, 高さ);
        mip_chain::記録する(device, command_buffer, image, 幅, 高さ, mip数);
    });

    // 安全性: 転送実行は完了済みで、ステージングバッファは以降使用しない。
    unsafe { device.destroy_buffer(ステージングバッファ, None) };
    device.メモリを解放する(ステージングメモリ);
    実行結果
}

fn 全レベルを転送先レイアウトへ遷移する(device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image, mip数: u32) {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(mip数)
        .base_array_layer(0)
        .layer_count(1);
    // 注意: mip0はこの直後のバッファ→画像コピー(COPY段)、mip1以降はミップチェーンの
    // blit(BLIT段)で書き込まれる。全レベルを1回のバリアでUNDEFINED解除するため、
    // dstStageMaskは両方を包含する`ALL_TRANSFER`にする。
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::TOP_OF_PIPE)
        .dst_stage_mask(vk::PipelineStageFlags2::ALL_TRANSFER)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲);
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは転送実行環境が記録用に開始済みで、imageは生成直後の
    // UNDEFINEDレイアウト。このバリアが一時コマンドバッファの先頭で唯一の書き手。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
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
    // 安全性: command_bufferは記録中。imageはTRANSFER_DST_OPTIMALへ遷移済み。
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
