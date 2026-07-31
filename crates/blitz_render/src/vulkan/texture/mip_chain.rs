//! vkCmdBlitImageの連鎖による縮小段チェーン生成。各段でTRANSFER_DST→TRANSFER_SRCの
//! バリア(`mip_barrier`)を積んでからLINEARフィルタでblitし、blit元として使い終えた
//! レベルはSHADER_READ_ONLY_OPTIMALへ遷移する(判断20)。
//!
//! 注意: blitの元/先の寸法は`max(1, 幅>>mip)`で計算する。1x1到達後も
//! この式が1を返し続けるため、途中でサイズが0になる事故を避けられる。

mod mip_barrier;

use ash::vk;

pub(super) fn 記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image, 幅: u32, 高さ: u32, mip数: u32) {
    for mip in 1..mip数 {
        mip_barrier::レベルをsrcへ遷移する(device, command_buffer, image, mip - 1);
        blitを積む(device, command_buffer, image, 幅, 高さ, mip);
        mip_barrier::レベルをshader_readへ遷移する(device, command_buffer, image, mip - 1);
    }
    mip_barrier::最終レベルをshader_readへ遷移する(device, command_buffer, image, mip数 - 1);
}

fn blitを積む(device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image, 幅: u32, 高さ: u32, mip: u32) {
    let 元寸法 = 寸法を求める(幅, 高さ, mip - 1);
    let 先寸法 = 寸法を求める(幅, 高さ, mip);
    let blit = vk::ImageBlit::default()
        .src_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(mip - 1)
                .base_array_layer(0)
                .layer_count(1),
        )
        .src_offsets([
            vk::Offset3D::default(),
            vk::Offset3D {
                x: 元寸法.0,
                y: 元寸法.1,
                z: 1,
            },
        ])
        .dst_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(mip)
                .base_array_layer(0)
                .layer_count(1),
        )
        .dst_offsets([
            vk::Offset3D::default(),
            vk::Offset3D {
                x: 先寸法.0,
                y: 先寸法.1,
                z: 1,
            },
        ]);
    // 安全性: command_bufferは記録中で、imageはこの直前のバリアでsrc/dstとも
    // 適切なレイアウトへ遷移済み。
    unsafe {
        device.cmd_blit_image(
            command_buffer,
            image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[blit],
            vk::Filter::LINEAR,
        );
    }
}

/// `max(1, 辺>>mip)`をi32へ変換する(Offset3Dはi32のため)。
fn 寸法を求める(幅: u32, 高さ: u32, mip: u32) -> (i32, i32) {
    let 幅段階 = (幅 >> mip).max(1);
    let 高さ段階 = (高さ >> mip).max(1);
    let 幅i32 = i32::try_from(幅段階).unwrap_or_else(|_| panic!("縮小段寸法がi32に収まらない: {幅段階}"));
    let 高さi32 = i32::try_from(高さ段階).unwrap_or_else(|_| panic!("縮小段寸法がi32に収まらない: {高さ段階}"));
    (幅i32, 高さi32)
}
