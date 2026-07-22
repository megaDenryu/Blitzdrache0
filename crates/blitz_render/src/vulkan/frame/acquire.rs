//! スワップチェーンからの次画像取得。

use ash::vk;

use crate::error::レンダラーエラー;

/// 次画像取得の結果。
pub(crate) enum 取得結果 {
    /// 画像を取得できた。`劣化` はサブオプティマル（描画は可能だが再構築が望ましい）。
    取得した { 添字: u32, 劣化: bool },
    /// スワップチェーンが陳腐化しており、取得できなかった。再構築が必要。
    再構築が必要,
}

pub(crate) fn 取得する(
    swapchain_loader: &ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    取得セマフォ: vk::Semaphore,
) -> Result<取得結果, レンダラーエラー> {
    // 安全性: swapchain・取得セマフォはいずれも生成済みで、呼び出し元が
    // フェンス待機によりセマフォが未シグナル状態であることを保証する。
    let 結果 = unsafe { swapchain_loader.acquire_next_image(swapchain, u64::MAX, 取得セマフォ, vk::Fence::null()) };

    match 結果 {
        Ok((添字, 劣化)) => Ok(取得結果::取得した { 添字, 劣化 }),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(取得結果::再構築が必要),
        Err(誤り) => Err(誤り.into()),
    }
}
