//! コマンド記録: レイアウト遷移 → dynamic renderingでのクリア → レイアウト遷移。

use ash::vk;

use super::barrier;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;

pub(super) fn コマンドを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
) -> Result<(), レンダラーエラー> {
    let begin_info = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferはRESET_COMMAND_BUFFERフラグ付きプール由来で、
    // ここでの開始が暗黙的に前回の記録をリセットする。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };

    let 部分範囲 = barrier::部分範囲を作る();
    barrier::描画前バリアを積む(device, command_buffer, 画像, 部分範囲);

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
    let rendering_info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: 寸法,
        })
        .layer_count(1)
        .color_attachments(&カラーアタッチメント一覧);

    // 安全性: command_bufferは記録中で、画像は事前バリアでCOLOR_ATTACHMENT_OPTIMALへ
    // 遷移済み。
    unsafe {
        device.cmd_begin_rendering(command_buffer, &rendering_info);
        device.cmd_end_rendering(command_buffer);
    }

    barrier::提示前バリアを積む(device, command_buffer, 画像, 部分範囲);

    // 安全性: command_bufferは記録開始済みで、対応するend呼び出し。
    unsafe { device.end_command_buffer(command_buffer)? };
    Ok(())
}
