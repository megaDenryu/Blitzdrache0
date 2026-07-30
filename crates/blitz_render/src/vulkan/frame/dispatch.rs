//! コマンド送信と提示へ渡す同期入力をまとめる。

use ash::vk;

use super::submit_present;
use super::{
    UI描画入力, シャドウ描画入力, ジオメトリ入力, スキニング描画入力, トーンマップ描画入力, ブルーム描画入力, 布描画入力, 空中遠近合成描画入力,
    空描画入力, 粒子描画入力,
};
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::大気LUT描画入力;
use crate::vulkan::swapchain::スワップチェーン画像添字;

pub(crate) struct 提示先<'a> {
    pub(crate) loader: &'a ash::khr::swapchain::Device,
    pub(crate) swapchain: vk::SwapchainKHR,
    pub(crate) 画像添字: スワップチェーン画像添字,
    /// 実表示時刻を計測しているときだけ`Some`。提示に付ける単調増加のID。
    pub(crate) 提示id: Option<u64>,
}

#[derive(Clone, Copy)]
pub(crate) struct 描画対象入力<'a> {
    pub(crate) ジオメトリ: &'a [ジオメトリ入力],
    pub(crate) シャドウ: &'a [シャドウ描画入力],
}

#[derive(Clone, Copy)]
pub(crate) struct 任意描画入力<'a> {
    /// 大気LUTを焼き直すフレームだけ`Some`。焼き直さないフレームは生成パスを1本も積まない。
    pub(crate) 大気lut: Option<&'a 大気LUT描画入力>,
    pub(crate) スキニング: Option<&'a スキニング描画入力>,
    pub(crate) 布: Option<&'a 布描画入力>,
    pub(crate) 空: Option<&'a 空描画入力>,
    /// 大気LUT腕で合成を切っていないフレームだけ`Some`。合成パスを空パスの前に1本積む。
    pub(crate) 空中遠近合成: Option<&'a 空中遠近合成描画入力>,
    pub(crate) 粒子: Option<&'a 粒子描画入力>,
    pub(crate) ブルーム: Option<&'a ブルーム描画入力>,
    pub(crate) トーンマップ: Option<&'a トーンマップ描画入力>,
    pub(crate) ui: Option<&'a UI描画入力>,
}

pub(crate) struct 同期入力 {
    pub(crate) 取得セマフォ: vk::Semaphore,
    pub(crate) 提示セマフォ: vk::Semaphore,
    pub(crate) 描画完了フェンス: vk::Fence,
}

pub(super) fn 送信して提示する(
    device: &ash::Device,
    queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    提示先: 提示先<'_>,
    同期: 同期入力,
    読み戻し待機が必要: bool,
) -> Result<bool, レンダラーエラー> {
    submit_present::送信して提示する(
        device,
        queue,
        command_buffer,
        提示先.loader,
        提示先.swapchain,
        提示先.画像添字,
        同期.取得セマフォ,
        同期.提示セマフォ,
        同期.描画完了フェンス,
        読み戻し待機が必要,
        提示先.提示id,
    )
}
