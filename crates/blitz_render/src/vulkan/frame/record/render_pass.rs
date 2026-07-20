//! dynamic renderingのカラー・深度アタッチメント記述と描画コマンドの発行。

use ash::vk;

use crate::vulkan::frame::{draw_commands, ジオメトリ入力};
use crate::clear_color::クリアカラー;

#[allow(clippy::too_many_arguments)]
pub(super) fn レンダリングを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像ビュー: vk::ImageView,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
) {
    let クリア値 = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: クリア色.rgba配列(),
        },
    };
    let カラーアタッチメント = vk::RenderingAttachmentInfo::default()
        .image_view(画像ビュー)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(クリア値);
    let カラーアタッチメント一覧 = [カラーアタッチメント];
    let 深度クリア値 = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 },
    };
    let 深度アタッチメント = vk::RenderingAttachmentInfo::default()
        .image_view(深度画像ビュー)
        .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::DONT_CARE)
        .clear_value(深度クリア値);
    let rendering_info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: 寸法,
        })
        .layer_count(1)
        .color_attachments(&カラーアタッチメント一覧)
        .depth_attachment(&深度アタッチメント);

    // 安全性: command_bufferは記録中で、画像・深度画像は事前バリアでそれぞれの
    // OPTIMALレイアウトへ遷移済み。
    unsafe {
        device.cmd_begin_rendering(command_buffer, &rendering_info);
        draw_commands::描画コマンドを積む(device, command_buffer, pipeline, 寸法, ジオメトリ入力);
        device.cmd_end_rendering(command_buffer);
    }
}
