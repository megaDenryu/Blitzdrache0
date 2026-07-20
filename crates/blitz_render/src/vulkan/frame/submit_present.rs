//! 記録済みコマンドバッファのsynchronization2送信と、スワップチェーンへの提示。

use ash::vk;

use crate::error::レンダラーエラー;

/// 戻り値: 提示が成功したときのサブオプティマル状態(true=再構築が望ましい)。
#[allow(clippy::too_many_arguments)]
pub(super) fn 送信して提示する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    swapchain_loader: &ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    画像添字: u32,
    取得セマフォ: vk::Semaphore,
    提示セマフォ: vk::Semaphore,
    描画完了フェンス: vk::Fence,
) -> Result<bool, レンダラーエラー> {
    let 待機セマフォ情報 = [vk::SemaphoreSubmitInfo::default()
        .semaphore(取得セマフォ)
        .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)];
    let コマンド情報 = [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
    let 通知セマフォ情報 = [vk::SemaphoreSubmitInfo::default()
        .semaphore(提示セマフォ)
        .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];

    let submit_info = vk::SubmitInfo2::default()
        .wait_semaphore_infos(&待機セマフォ情報)
        .command_buffer_infos(&コマンド情報)
        .signal_semaphore_infos(&通知セマフォ情報);

    // 安全性: command_bufferは記録済み。描画完了フェンスはこのフレーム開始時に
    // リセット済みで、GPU完了検知に使う唯一の待機対象。
    unsafe { device.queue_submit2(queue, &[submit_info], 描画完了フェンス)? };

    let スワップチェーン一覧 = [swapchain];
    let 画像添字一覧 = [画像添字];
    let 提示待機セマフォ一覧 = [提示セマフォ];
    let present_info = vk::PresentInfoKHR::default()
        .wait_semaphores(&提示待機セマフォ一覧)
        .swapchains(&スワップチェーン一覧)
        .image_indices(&画像添字一覧);

    // 安全性: 提示セマフォはこの送信のsignal対象で、GPU側の描画完了後にシグナルされる。
    let 提示結果 = unsafe { swapchain_loader.queue_present(queue, &present_info) };
    match 提示結果 {
        Ok(劣化) => Ok(劣化),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(true),
        Err(誤り) => Err(誤り.into()),
    }
}
