//! 送信が成った後の提示の工程。受け取るのは提示先と同期資源と読み戻しの待機が要るかどうか、返すのはそのフレームの`送信後の結末`である。
//!
//! 送信の局面と分けるのは、ここから先の失敗がGPUへコマンドを渡した後に起きるためであり、呼び出し元の後始末が変わる。
//! スワップチェーンの陳腐化(`ERROR_OUT_OF_DATE_KHR`)を提示劣化へ畳むのもこの工程が持つ。

use ash::vk;

use super::submit_outcome::送信後の結末;
use super::{同期入力, 提示先};

pub(super) fn 待って提示する(
    device: &ash::Device,
    queue: vk::Queue,
    提示先: 提示先<'_>,
    同期: &同期入力,
    読み戻し待機が必要: bool,
) -> 送信後の結末 {
    if 読み戻し待機が必要 {
        // 安全性: 直前に送信した同じフェンスを待つ。読み戻しバッファへのコピー完了をホストが読む前に保証するため、
        // 通常経路と異なりここで同期的に待機する(判断9: GPU同期を伴うためスモーク用途)。
        if let Err(誤り) = unsafe { device.wait_for_fences(&[同期.描画完了フェンス], true, u64::MAX) } {
            return 送信後の結末::送信後に失敗(誤り.into());
        }
    }

    let スワップチェーン一覧 = [提示先.swapchain];
    let 画像添字一覧 = [提示先.画像添字.gpu境界用u32()];
    let 提示待機セマフォ一覧 = [同期.提示セマフォ];
    let 提示結果 = 提示する(
        提示先.loader,
        queue,
        &提示待機セマフォ一覧,
        &スワップチェーン一覧,
        &画像添字一覧,
        提示先.提示id,
    );
    match 提示結果 {
        Ok(提示劣化) => 送信後の結末::提示まで成功 { 提示劣化 },
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => 送信後の結末::提示まで成功 { 提示劣化: true },
        Err(誤り) => 送信後の結末::送信後に失敗(誤り.into()),
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
