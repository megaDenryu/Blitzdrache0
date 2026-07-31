//! コマンドバッファを1本取り、渡された記録を書いて送信し、フェンスで完了を待つ工程。
//! 受け取るのは記録のクロージャ、返り値は成否だけである。呼び出しから戻った時点でGPUの作業は終わっている。
//!
//! 検査だけがこの経路を使う。進行中フレームも提示も持たないため、送信ごとに待つことが許される。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 実行する(
    device: &GPUデバイス,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
    記録: impl FnOnce(&ash::Device, vk::CommandBuffer),
) -> Result<(), レンダラーエラー> {
    let 割当情報 = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    // 安全性: command_poolは生成済みで有効。
    let 一覧 = unsafe { device.allocate_command_buffers(&割当情報)? };
    let Some(&command_buffer) = 一覧.first() else {
        panic!("allocate_command_buffersが成功したのにコマンドバッファが0本だった(Vulkan実装の契約違反)");
    };
    let 結果 = 記録して待つ(device, queue, command_buffer, 記録);
    // 安全性: command_bufferはこの関数が唯一の所有者であり、送信の完了をフェンスで待った後に解放する。
    unsafe { device.free_command_buffers(command_pool, &[command_buffer]) };
    結果
}

fn 記録して待つ(
    device: &GPUデバイス,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    記録: impl FnOnce(&ash::Device, vk::CommandBuffer),
) -> Result<(), レンダラーエラー> {
    let begin_info = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferは割当直後で記録中でない。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };
    記録(device, command_buffer);
    // 安全性: command_bufferは記録開始済みで、対応するend呼び出しである。
    unsafe { device.end_command_buffer(command_buffer)? };

    let フェンス生成情報 = vk::FenceCreateInfo::default();
    // 安全性: deviceは生成済みで有効。
    let フェンス = unsafe { device.create_fence(&フェンス生成情報, None)? };
    let 結果 = 送信して待つ(device, queue, command_buffer, フェンス);
    // 安全性: フェンスはこの関数が唯一の所有者であり、待機の完了後に破棄する。
    unsafe { device.destroy_fence(フェンス, None) };
    結果
}

fn 送信して待つ(
    device: &GPUデバイス,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    フェンス: vk::Fence,
) -> Result<(), レンダラーエラー> {
    let コマンドバッファ情報 = [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
    let 送信情報 = [vk::SubmitInfo2::default().command_buffer_infos(&コマンドバッファ情報)];
    // 安全性: command_bufferは記録済みで、queueとフェンスは生成済みで有効。
    unsafe { device.queue_submit2(queue, &送信情報, フェンス)? };
    // 安全性: フェンスは直前の送信に結び付けた唯一のものである。
    unsafe { device.wait_for_fences(&[フェンス], true, u64::MAX)? };
    Ok(())
}
