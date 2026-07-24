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
    読み戻し待機が必要: bool,
    提示id: Option<u64>,
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

    if 読み戻し待機が必要 {
        // 安全性: 直前に送信した同じフェンスを待つ。読み戻しバッファへのコピー完了を
        // ホストが読む前に保証するため、通常経路と異なりここで同期的に待機する
        // （判断9: GPU同期を伴うためスモーク用途）。
        unsafe { device.wait_for_fences(&[描画完了フェンス], true, u64::MAX)? };
    }

    let スワップチェーン一覧 = [swapchain];
    let 画像添字一覧 = [画像添字];
    let 提示待機セマフォ一覧 = [提示セマフォ];
    let 提示結果 = 提示する(
        swapchain_loader,
        queue,
        &提示待機セマフォ一覧,
        &スワップチェーン一覧,
        &画像添字一覧,
        提示id,
    );
    match 提示結果 {
        Ok(劣化) => Ok(劣化),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(true),
        Err(誤り) => Err(誤り.into()),
    }
}

/// 実表示時刻を計測するときだけ`VkPresentIdKHR`を連結する。IDを付けない経路では連結自体を行わない。
fn 提示する(
    swapchain_loader: &ash::khr::swapchain::Device,
    queue: vk::Queue,
    提示待機セマフォ一覧: &[vk::Semaphore],
    スワップチェーン一覧: &[vk::SwapchainKHR],
    画像添字一覧: &[u32],
    提示id: Option<u64>,
) -> ash::prelude::VkResult<bool> {
    let 基本情報 = vk::PresentInfoKHR::default()
        .wait_semaphores(提示待機セマフォ一覧)
        .swapchains(スワップチェーン一覧)
        .image_indices(画像添字一覧);
    let Some(識別子) = 提示id else {
        // 安全性: 提示セマフォはこの送信のsignal対象で、GPU側の描画完了後にシグナルされる。
        return unsafe { swapchain_loader.queue_present(queue, &基本情報) };
    };
    let 提示id一覧 = [識別子];
    let mut 提示id情報 = vk::PresentIdKHR::default().present_ids(&提示id一覧);
    let 提示情報 = 基本情報.push_next(&mut 提示id情報);
    // 安全性: 上と同じ条件に加え、連結した提示ID情報とID配列はこの呼び出しの間スタック上で生存する。
    unsafe { swapchain_loader.queue_present(queue, &提示情報) }
}
