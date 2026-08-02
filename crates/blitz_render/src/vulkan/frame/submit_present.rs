//! 記録済みコマンドバッファのsynchronization2送信と、送信を境にしたフレームの結末の組み立て。提示の局面は`present`が持つ。
//!
//! 返す型を2段にするのは、送信の前と後で失敗の意味が変わるためである。送信へ到達しなかった失敗は`Err`であり、
//! そのフレームはGPUへ何も渡していない。送信が成った後の失敗は`送信後の結末`に載せ、呼び出し元が資源表世代の保持を
//! 記録してから失敗を返せるようにする。

use ash::vk;

use super::present;
use super::submit_outcome::送信後の結末;
use super::{同期入力, 提示先};
use crate::error::レンダラーエラー;

/// `Err`は送信へ到達しなかったことだけを表す。呼び出し元はこの区別で、そのフレームが資源表世代を保持したかどうかを決める。
pub(super) fn 送信して提示する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    提示先: 提示先<'_>,
    同期: &同期入力,
    読み戻し待機が必要: bool,
) -> Result<送信後の結末, レンダラーエラー> {
    送信する(device, queue, command_buffer, 同期)?;
    Ok(present::待って提示する(device, queue, 提示先, 同期, 読み戻し待機が必要))
}

/// 注意: フェンスの無信号化からqueue_submit2までの間で失敗すると、そのフェンスは誰もシグナルしないまま残る。
/// 次に同じフレームスロットを待つ側は永久に返らないため、この区間の失敗は回復できない。呼び出し元はフレームループを畳んで終える。
fn 送信する(
    device: &ash::Device, queue: vk::Queue, command_buffer: vk::CommandBuffer, 同期: &同期入力
) -> Result<(), レンダラーエラー> {
    let 待機セマフォ情報 = [vk::SemaphoreSubmitInfo::default()
        .semaphore(同期.取得セマフォ)
        .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)];
    let コマンド情報 = [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
    let 通知セマフォ情報 = [vk::SemaphoreSubmitInfo::default()
        .semaphore(同期.提示セマフォ)
        .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];

    let submit_info = vk::SubmitInfo2::default()
        .wait_semaphore_infos(&待機セマフォ情報)
        .command_buffer_infos(&コマンド情報)
        .signal_semaphore_infos(&通知セマフォ情報);

    // 注意: フェンスの無信号化を送信の直前に置く。フレームの準備の途中で失敗すると送信へ到達せず、
    // 先に無信号化していたフェンスは永久にsignalされない。次に同じフレームスロットを使う待機がそこで止まる。
    // 安全性: このフェンスはこのフレームの先頭で待機済みであり、待機から送信までの間に誰も使わない。
    unsafe { device.reset_fences(&[同期.描画完了フェンス])? };
    // 安全性: command_bufferは記録済み。描画完了フェンスは直前で無信号化済みで、GPU完了検知に使う唯一の待機対象。
    unsafe { device.queue_submit2(queue, &[submit_info], 同期.描画完了フェンス)? };
    Ok(())
}
