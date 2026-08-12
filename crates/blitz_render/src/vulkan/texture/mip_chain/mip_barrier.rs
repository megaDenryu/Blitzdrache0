//! 縮小段チェーン生成中の各レベルのレイアウト遷移バリア(synchronization2)。

use ash::vk;

use crate::vulkan::command_sink::GPU命令の積み先;

pub(super) fn 部分範囲(レベル: u32) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(レベル)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

fn 積む(積み先: GPU命令の積み先<'_>, バリア: vk::ImageMemoryBarrier2) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは転送実行環境が記録用に開始済みで、imageは生成済み。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}

/// TRANSFER_DST_OPTIMAL(コピーまたはblitで書き込み済み) → TRANSFER_SRC_OPTIMAL。
/// 次段のblit元にする前に積む。
///
/// 注意: このレベルへの直前の書き込みはmip0ならバッファ→画像コピー(COPY段)、
/// mip1以降なら前段のblit(BLIT段)であり、呼び出し側でどちらか判別しない。
/// `ALL_TRANSFER`(COPY/BLIT両方を包含)をsrcStageMaskにすることで、
/// 実際の書き手がどちらであっても同期スコープに含める。
pub(super) fn レベルをsrcへ遷移する(積み先: GPU命令の積み先<'_>, image: vk::Image, レベル: u32) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::ALL_TRANSFER)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::BLIT)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲(レベル));
    積む(積み先, バリア);
}

/// TRANSFER_SRC_OPTIMAL(blit元として使用済み) → SHADER_READ_ONLY_OPTIMAL。
pub(super) fn レベルをshader_readへ遷移する(積み先: GPU命令の積み先<'_>, image: vk::Image, レベル: u32) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::BLIT)
        .src_access_mask(vk::AccessFlags2::TRANSFER_READ)
        .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::SHADER_READ)
        .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲(レベル));
    積む(積み先, バリア);
}

/// 最終縮小段レベル専用: TRANSFER_DST_OPTIMAL(blitの先として使用済み、以降blit元には
/// ならない) → SHADER_READ_ONLY_OPTIMAL。
///
/// 注意: mip数が1(縮小段無し)ならこのレベルはmip0そのものでCOPY段の書き込み、
/// mip数が2以上ならblit(BLIT段)の書き込み。`レベルをsrcへ遷移する`と同じ理由で
/// `ALL_TRANSFER`を使う。
pub(super) fn 最終レベルをshader_readへ遷移する(積み先: GPU命令の積み先<'_>, image: vk::Image, レベル: u32) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::ALL_TRANSFER)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::SHADER_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(部分範囲(レベル));
    積む(積み先, バリア);
}
