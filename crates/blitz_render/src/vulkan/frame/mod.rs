//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。

mod barrier;
mod record;
mod submit_present;

pub(crate) mod acquire;

pub(crate) use acquire::取得結果;

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;

/// 取得済みの画像に対して1フレーム分のコマンドを記録し、送信・提示する。
/// 戻り値は「提示まで到達したか（true）／スワップチェーンが陳腐化していたか（false）」。
#[allow(clippy::too_many_arguments)]
pub(crate) fn 描画する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    swapchain_loader: &ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    画像添字: u32,
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    取得セマフォ: vk::Semaphore,
    提示セマフォ: vk::Semaphore,
    描画完了フェンス: vk::Fence,
) -> Result<bool, レンダラーエラー> {
    record::コマンドを記録する(device, command_buffer, 画像, 画像ビュー, 寸法, クリア色)?;
    submit_present::送信して提示する(
        device,
        queue,
        command_buffer,
        swapchain_loader,
        swapchain,
        画像添字,
        取得セマフォ,
        提示セマフォ,
        描画完了フェンス,
    )
}
