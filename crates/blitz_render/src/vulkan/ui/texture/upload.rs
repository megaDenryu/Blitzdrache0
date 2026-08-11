//! ステージングバッファ経由でUIテクスチャの唯一の縮小段レベルへ転送する
//! (マテリアルテクスチャと異なり縮小段チェーン生成は行わない)。レイアウト遷移
//! バリアの組み立ては`barrier`に委ねる。

mod barrier;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::transfer::転送実行環境;

pub(super) fn ホストの画素列を画像へ転送する(
    確保係: &GPU資源の確保係<'_>,
    転送環境: &転送実行環境,
    image: vk::Image,
    幅: u32,
    高さ: u32,
    rgba8: &[u8],
) -> Result<(), レンダラーエラー> {
    let ステージング = 確保係.ホスト可視バッファを確保して書き込む(rgba8, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let 実行結果 =
        ステージングバッファから唯一の縮小段レベルへ転送する(転送環境, ステージング.バッファのハンドル(), image, 幅, 高さ);

    ステージング.破棄する(確保係.論理デバイス());
    実行結果
}

fn ステージングバッファから唯一の縮小段レベルへ転送する(
    転送環境: &転送実行環境,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    幅: u32,
    高さ: u32,
) -> Result<(), レンダラーエラー> {
    let 一時 = 転送環境.転送コマンドを積み始める()?;
    let device = 一時.論理デバイス();
    let command_buffer = 一時.積む先のコマンドバッファ();
    barrier::転送先へ遷移する(device, command_buffer, image);
    コピーする(device, command_buffer, ステージングバッファ, image, 幅, 高さ);
    barrier::シェーダー読み取り専用へ遷移する(device, command_buffer, image);
    一時.送信して完了を待つ()
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
