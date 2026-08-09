//! 材質テクスチャの転送が積む、画像レイアウト遷移バリアの組み立て。担当する工程は
//! 「全縮小段レベルを転送先レイアウトへ移すバリアと、コピーで積み終えた全レベルを読み取り専用へ移すバリアを積む」ことであり、
//! 受け取るのは積み先のコマンドバッファと画像と縮小段の数、積んだ後に返すものは無い。
//!
//! 触れるのは引数で受け取った画像1枚だけであり、この工程はバッファもメモリも一切知らない。

use ash::vk;

fn 全レベルの部分範囲(縮小段数: u32) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(縮小段数)
        .base_array_layer(0)
        .layer_count(1)
}

pub(super) fn 全レベルを転送先レイアウトへ遷移する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    縮小段数: u32,
) {
    let 部分範囲 = 全レベルの部分範囲(縮小段数);
    // 注意: 積み方によって書き手が変わる。GPUのblitで縮小段を作る側は段0がバッファ→画像コピー(COPY段)で
    // 段1以降がblit(BLIT段)、全段を転送する側は全段がCOPY段である。全レベルを1回のバリアでUNDEFINED解除するため、
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

/// TRANSFER_DST_OPTIMAL(全段をコピーで書き込み済み) → SHADER_READ_ONLY_OPTIMAL。
/// 全段をファイルから転送する積み方だけが使う。段ごとに分けず1回で移すのは、この積み方ではどの段もblitの元にならず、
/// 段と段の間に順序の依存が1つも無いためである。
pub(super) fn 全レベルをshader_readへ遷移する(
    device: &ash::Device, command_buffer: vk::CommandBuffer, image: vk::Image, 縮小段数: u32
) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COPY)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::SHADER_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(全レベルの部分範囲(縮小段数));
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは積み込み中で、直前のコピーが全レベルへ書き終えている。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
