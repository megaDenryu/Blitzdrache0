//! コマンドプールと、フレームインフライト数ぶんのプライマリコマンドバッファの生成。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームインフライト数;

pub(crate) fn 生成する(
    device: &ash::Device,
    キューファミリ添字: u32,
) -> Result<(vk::CommandPool, [vk::CommandBuffer; フレームインフライト数]), レンダラーエラー> {
    let プール生成情報 = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(キューファミリ添字);

    // 安全性: deviceは生成済みで有効。キューファミリ添字は選定済みの正当な値。
    let command_pool = unsafe { device.create_command_pool(&プール生成情報, None)? };

    let 割当数 = u32::try_from(フレームインフライト数)
        .unwrap_or_else(|_| panic!("フレームインフライト数がu32に収まらない"));
    let 割当情報 = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(割当数);

    // 安全性: command_poolは直前に生成済みで、このスコープの唯一の所有者はこの関数。
    let command_buffer一覧 = unsafe { device.allocate_command_buffers(&割当情報)? };
    let Ok(command_buffer一覧): Result<[vk::CommandBuffer; フレームインフライト数], _> =
        command_buffer一覧.try_into()
    else {
        // command_buffer_count(フレームインフライト数)を要求してVulkanが成功を返したのに
        // 要求数ぴったりで得られないのは、Vulkan実装がその契約を破っている状態であり回復不能。
        panic!("allocate_command_buffersがフレームインフライト数ぴったりのコマンドバッファを返さなかった");
    };

    Ok((command_pool, command_buffer一覧))
}
