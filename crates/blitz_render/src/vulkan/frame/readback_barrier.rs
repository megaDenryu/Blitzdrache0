//! 読み戻し用の追加レイアウト遷移(synchronization2)。
//! COLOR_ATTACHMENT_OPTIMAL ⇔ TRANSFER_SRC_OPTIMAL。判断9のピクセル読み戻しでのみ使う。

use ash::vk;

/// COLOR_ATTACHMENT_OPTIMAL → TRANSFER_SRC_OPTIMAL。コピー前に積む。
pub(super) fn コピー前バリアを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    部分範囲: vk::ImageSubresourceRange,
) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::COPY)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(画像)
        .subresource_range(部分範囲);
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは記録中で、画像は描画済みのスワップチェーン画像。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}

/// TRANSFER_SRC_OPTIMAL → PRESENT_SRC_KHR。コピー後・提示前に積む。
pub(super) fn コピー後バリアを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    部分範囲: vk::ImageSubresourceRange,
) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COPY)
        .src_access_mask(vk::AccessFlags2::TRANSFER_READ)
        .dst_stage_mask(vk::PipelineStageFlags2::BOTTOM_OF_PIPE)
        .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(画像)
        .subresource_range(部分範囲);
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは記録中で、画像はコピー完了済み。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
