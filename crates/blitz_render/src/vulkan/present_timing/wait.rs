//! 提示の表示完了待ち。まずtimeout=0で判定し、未表示のときだけ待機上限つきで待ち直す。

use std::time::Instant;

use ash::vk;

use super::record::{待機結末, 表示時刻記録};
use crate::error::レンダラーエラー;

/// 待ち直すときの上限。表示が完全に止まった環境で描画ループを永久に止めないための保険である。
const 待機上限NS: u64 = 1_000_000_000;

/// 外部由来のVkResultを、この計測が区別する3つの状態へ畳んでから分岐する。
enum 待機判定 {
    表示された,
    まだ表示されていない,
    スワップチェーンが陳腐化した,
}

/// 注意: 待機対象は最後に発番したIDより1つ古い提示に固定してある(参照: `record`の待機対象の戻り数)。
/// timeout=0での判定を先に置き、描画ループを止める必要があるときだけ止める設計だが、
/// 実測ではNVIDIAドライバー(596.21)がtimeout=0でも表示まで停止した。停止した長さは観測ごとの停止時間msに残るため、
/// 観測時刻が実表示時刻かどうかは停止時間msの分布で判定すること。
pub(super) fn 表示を待って記録する(
    待機: &ash::khr::present_wait::Device,
    記録: &mut 表示時刻記録,
    swapchain: vk::SwapchainKHR,
) -> Result<(), レンダラーエラー> {
    let Some(対象id) = 記録.待機対象id() else {
        return Ok(());
    };
    let 開始 = Instant::now();
    // 安全性: swapchainは現行のもので、待機ローダーはその生成元デバイスから作られている。
    let 即時判定 = 判定する(unsafe { 待機.wait_for_present(swapchain, 対象id, 0) })?;
    match 即時判定 {
        待機判定::表示された => {
            記録.観測を加える(
                待機結末::表示された {
                    停止時間ms: 経過ms(開始)
                },
                Instant::now(),
            );
            Ok(())
        }
        待機判定::まだ表示されていない => 待ち直して観測する(待機, 記録, swapchain, 対象id, 開始),
        待機判定::スワップチェーンが陳腐化した => {
            記録.提示の追跡をやり直す();
            Ok(())
        }
    }
}

fn 待ち直して観測する(
    待機: &ash::khr::present_wait::Device,
    記録: &mut 表示時刻記録,
    swapchain: vk::SwapchainKHR,
    対象id: u64,
    開始: Instant,
) -> Result<(), レンダラーエラー> {
    // 安全性: 即時判定と同じ引数であり、swapchainとローダーの対応は呼び出し元が保証する。
    let 判定 = 判定する(unsafe { 待機.wait_for_present(swapchain, 対象id, 待機上限NS) })?;
    let 完了 = Instant::now();
    let 停止時間ms = 完了.duration_since(開始).as_secs_f64() * 1000.0;
    match 判定 {
        待機判定::表示された => 記録.観測を加える(待機結末::表示された { 停止時間ms }, 完了),
        待機判定::まだ表示されていない => 記録.観測を加える(待機結末::時間切れになった { 停止時間ms }, 完了),
        待機判定::スワップチェーンが陳腐化した => 記録.提示の追跡をやり直す(),
    }
    Ok(())
}

fn 経過ms(開始: Instant) -> f64 {
    Instant::now().duration_since(開始).as_secs_f64() * 1000.0
}

fn 判定する(結果: ash::prelude::VkResult<()>) -> Result<待機判定, レンダラーエラー> {
    match 結果 {
        // 劣化(SUBOPTIMAL)は表示自体には成功しているため、表示された側へ畳む。
        Ok(()) | Err(vk::Result::SUBOPTIMAL_KHR) => Ok(待機判定::表示された),
        Err(vk::Result::TIMEOUT) => Ok(待機判定::まだ表示されていない),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(待機判定::スワップチェーンが陳腐化した),
        Err(誤り) => Err(誤り.into()),
    }
}
