//! 一時コマンドバッファの記録・送信・完了待機。呼び出し元が渡す`転送コマンドを積む`クロージャで
//! 転送コマンド(コピー・バリア・blit)を積み、グラフィックスキューへsubmitして
//! fence待ちで完了を保証する(判断2の一時コマンドバッファヘルパー)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 一括実行する(
    device: &ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    転送コマンドを積む: impl FnOnce(vk::CommandBuffer),
) -> Result<(), レンダラーエラー> {
    let alloc_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    // 安全性: command_poolは生成済みで有効。
    let command_buffer一覧 = unsafe { device.allocate_command_buffers(&alloc_info)? };
    let Some(&command_buffer) = command_buffer一覧.first() else {
        // command_buffer_count(1)を要求してVulkanが成功を返したのに0本なのは
        // Vulkan実装がその契約を破っている状態であり回復不能。
        panic!("allocate_command_buffersが1本のコマンドバッファを返さなかった");
    };

    let 実行結果 = 記録して送信する(device, queue, command_buffer, 転送コマンドを積む);

    // 安全性: command_bufferはこのスコープの唯一の所有者で、送信完了(fence待ち済み)
    // または送信前エラーのいずれの場合も以降使用しない。
    unsafe { device.free_command_buffers(command_pool, &[command_buffer]) };
    実行結果
}

fn 記録して送信する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    転送コマンドを積む: impl FnOnce(vk::CommandBuffer),
) -> Result<(), レンダラーエラー> {
    let begin_info = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferは直前に確保済みで未記録状態。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };
    転送コマンドを積む(command_buffer);
    // 安全性: command_bufferは記録開始済みで、対応するend呼び出し。
    unsafe { device.end_command_buffer(command_buffer)? };

    let fence_create_info = vk::FenceCreateInfo::default();
    // 安全性: deviceは生成済みで有効。
    let fence = unsafe { device.create_fence(&fence_create_info, None)? };

    let command_buffer一覧 = [command_buffer];
    let submit_info = vk::SubmitInfo::default().command_buffers(&command_buffer一覧);
    // 安全性: command_bufferは記録済みで、fenceは直前に生成した非シグナル状態。
    let 送信結果 = unsafe { device.queue_submit(queue, &[submit_info], fence) };
    let 待機結果 = 送信結果.and_then(|()| {
        // 安全性: fenceはこの送信の完了を示す唯一の待機対象。
        unsafe { device.wait_for_fences(&[fence], true, u64::MAX) }
    });
    // 安全性: fenceはこのスコープの唯一の所有者で、待機の成否によらず以降使用しない。
    unsafe { device.destroy_fence(fence, None) };
    待機結果.map_err(Into::into)
}
