//! グラフィックスパスのdynamic rendering開始/終了。アタッチメント記述は
//! パス種別の宣言(カラー・深度・クリア指定)から実行器が組み立てる
//! （判断28: begin/end renderingは実行器の責務、クロージャはバインド+ドローのみ）。

use ash::vk;

use super::clear_spec::クリア指定;
use super::handle::画像ハンドル;
use super::registry::画像レジストリ;

pub(crate) fn 開始する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    レジストリ: &画像レジストリ,
    カラー: 画像ハンドル,
    深度: Option<画像ハンドル>,
    クリア指定: &クリア指定,
    寸法: vk::Extent2D,
) {
    let カラークリア値 = vk::ClearValue {
        color: vk::ClearColorValue { float32: クリア指定.カラー.rgba配列() },
    };
    let カラーアタッチメント = vk::RenderingAttachmentInfo::default()
        .image_view(レジストリ.ビューを取得する(カラー))
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(カラークリア値);
    let カラーアタッチメント一覧 = [カラーアタッチメント];

    let 深度クリア値 = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 },
    };
    let 深度アタッチメント = 深度.map(|深度ハンドル| {
        vk::RenderingAttachmentInfo::default()
            .image_view(レジストリ.ビューを取得する(深度ハンドル))
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .clear_value(深度クリア値)
    });

    let mut rendering_info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: 寸法 })
        .layer_count(1)
        .color_attachments(&カラーアタッチメント一覧);
    if let Some(深度アタッチメント) = &深度アタッチメント {
        rendering_info = rendering_info.depth_attachment(深度アタッチメント);
    }

    // 安全性: command_bufferは記録中で、各画像は直前のバリア発行でOPTIMALレイアウトへ
    // 遷移済み。
    unsafe { device.cmd_begin_rendering(command_buffer, &rendering_info) };
}

pub(crate) fn 終了する(device: &ash::Device, command_buffer: vk::CommandBuffer) {
    // 安全性: 対応する開始する呼び出しで記録中のrenderingを閉じる。
    unsafe { device.cmd_end_rendering(command_buffer) };
}
