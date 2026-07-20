//! UIテクスチャの唯一のミップレベルに対するレイアウト遷移バリア2つ
//! (UNDEFINED→TRANSFER_DST、TRANSFER_DST→SHADER_READ_ONLY)。

use ash::vk;

fn 部分範囲() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

pub(super) fn 転送先へ遷移する(device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::TOP_OF_PIPE)
        .dst_stage_mask(vk::PipelineStageFlags2::COPY)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲());
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは転送実行環境が記録用に開始済みで、imageは生成直後の
    // UNDEFINEDレイアウト。このバリアが唯一の書き手。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}

pub(super) fn シェーダー読み取り専用へ遷移する(device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COPY)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::SHADER_SAMPLED_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲());
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは記録中で、imageは直前のコピーでTRANSFER_DST_OPTIMAL。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
