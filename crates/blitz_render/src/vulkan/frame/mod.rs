//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。

mod cloth_types;
mod copy;
mod images;
mod record;
mod submit_present;
mod types;

pub(crate) mod acquire;
pub(crate) mod draw_commands;

pub(crate) use acquire::取得結果;
pub(crate) use cloth_types::布描画入力;
pub(crate) use images::{ブルーム画像, フレーム画像一式};
pub(crate) use types::{
    シャドウ描画入力, スキニング描画入力, ジオメトリ入力, トーンマップ描画入力, ブルーム描画入力,
    描画方式, 粒子描画入力, UI描画入力, UI描画項目,
};

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;

/// 取得済みの画像に対して1フレーム分のコマンドを記録し、送信・提示する。
/// 戻り値は「提示まで到達したか（true）／スワップチェーンが陳腐化していたか（false）」と、
/// このフレームで書いたGPUタイムスタンプの「パス名→クエリ開始添字」対応(判断30)。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn 描画する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    swapchain_loader: &ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    画像添字: u32,
    画像一式: &フレーム画像一式,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
    シャドウ入力: &シャドウ描画入力,
    スキニング入力: Option<&スキニング描画入力>,
    布入力: Option<&布描画入力>,
    粒子入力: Option<&粒子描画入力>,
    ブルーム入力: Option<&ブルーム描画入力>,
    トーンマップ入力: Option<&トーンマップ描画入力>,
    ui入力: Option<&UI描画入力>,
    描画方式: 描画方式,
    クエリプール: Option<vk::QueryPool>,
    取得セマフォ: vk::Semaphore,
    提示セマフォ: vk::Semaphore,
    描画完了フェンス: vk::Fence,
) -> Result<(bool, Vec<(&'static str, u32)>), レンダラーエラー> {
    let 読み戻し待機が必要 = matches!(描画方式, 描画方式::読み戻し { .. });
    let 計測マッピング = record::コマンドを記録する(
        device,
        command_buffer,
        画像一式,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
        シャドウ入力,
        スキニング入力,
        布入力,
        粒子入力,
        ブルーム入力,
        トーンマップ入力,
        ui入力,
        &描画方式,
        クエリプール,
    )?;
    let 提示劣化 = submit_present::送信して提示する(
        device,
        queue,
        command_buffer,
        swapchain_loader,
        swapchain,
        画像添字,
        取得セマフォ,
        提示セマフォ,
        描画完了フェンス,
        読み戻し待機が必要,
    )?;
    Ok((提示劣化, 計測マッピング))
}
