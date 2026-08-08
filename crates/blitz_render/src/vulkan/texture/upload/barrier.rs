//! 材質テクスチャの転送が積む、画像レイアウト遷移バリアの組み立て。担当する工程は
//! 「確保直後の全縮小段レベルを転送先レイアウトへ移すバリアを1つ積む」ことであり、受け取るのは
//! 積み先のコマンドバッファと画像と縮小段の数、積んだ後に返すものは無い。
//!
//! 触れるのは引数で受け取った画像1枚だけであり、この工程はバッファもメモリも一切知らない。

use ash::vk;

pub(super) fn 全レベルを転送先レイアウトへ遷移する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    mip数: u32,
) {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(mip数)
        .base_array_layer(0)
        .layer_count(1);
    // 注意: mip0はこの直後のバッファ→画像コピー(COPY段)、mip1以降は縮小段チェーンの
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
    // 安全性: command_bufferは転送実行環境が積み込み用に開始済みで、imageは生成直後の
    // UNDEFINEDレイアウト。このバリアが一時コマンドバッファの先頭で唯一の書き手。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
