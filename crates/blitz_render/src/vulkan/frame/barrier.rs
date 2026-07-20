//! カラーアタッチメント用のレイアウト遷移バリア(synchronization2)。

use ash::vk;

pub(super) fn 部分範囲を作る() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

/// UNDEFINED → COLOR_ATTACHMENT_OPTIMAL。描画開始前に積む。
pub(super) fn 描画前バリアを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    部分範囲: vk::ImageSubresourceRange,
) {
    // 注意: srcStageMaskはTOP_OF_PIPEではなくCOLOR_ATTACHMENT_OUTPUTにする。
    // 取得セマフォのwait_semaphore_infoはCOLOR_ATTACHMENT_OUTPUT段で待つため、
    // このバリアがそれより早い段(TOP_OF_PIPE)だと待機のスコープ外になり、
    // vkAcquireNextImageKHRの読み出しに対してWRITE_AFTER_READ検証エラーになる。
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(画像)
        .subresource_range(部分範囲);
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは記録中で、画像はこのフレームで取得済みのスワップチェーン画像。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}

/// COLOR_ATTACHMENT_OPTIMAL → PRESENT_SRC_KHR。描画終了後・提示前に積む。
pub(super) fn 提示前バリアを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    部分範囲: vk::ImageSubresourceRange,
) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::BOTTOM_OF_PIPE)
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(画像)
        .subresource_range(部分範囲);
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは記録中で、画像はこのフレームで描画完了済み。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
