//! 次の提示画像を取得し、再構築が必要な場合は見送りを返す。

use super::super::cpu_timing::CPU区間時計;
use super::super::レンダラー;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::frame::取得結果;

pub(super) fn 実行する(
    レンダラー: &mut レンダラー,
    フレーム添字: usize,
    cpu区間時計: &mut Option<CPU区間時計>,
) -> Result<Option<u32>, レンダラーエラー> {
    let 取得セマフォ = レンダラー.フレーム同期.取得セマフォ(フレーム添字);
    let 結果 = vulkan::frame::acquire::取得する(&レンダラー.swapchain_loader, レンダラー.swapchain.handle, 取得セマフォ)?;
    let 添字 = match 結果 {
        取得結果::取得した { 添字, 劣化 } => {
            レンダラー.再構築が必要 |= 劣化;
            Some(添字)
        }
        取得結果::再構築が必要 => {
            レンダラー.再構築が必要 = true;
            None
        }
    };
    if let Some(時計) = cpu区間時計 {
        時計.画像取得を終了する();
    }
    Ok(添字)
}
