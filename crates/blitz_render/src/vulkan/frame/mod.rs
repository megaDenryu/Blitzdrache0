//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。

mod copy;
mod draw_commands;
mod record;
mod submit_present;

pub(crate) mod acquire;

pub(crate) use acquire::取得結果;

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;

/// このフレームの描画後処理: 通常の提示前遷移のみか、読み戻し用のコピーを挟むか。
pub(crate) enum 描画方式 {
    通常,
    読み戻し { バッファ: vk::Buffer },
}

/// 頂点/インデックスバッファと、マテリアルテクスチャ+フレームユニフォームを
/// 束ねたディスクリプタセット。ビュー射影行列等はUBO(判断24)経由で渡すため
/// ここには含まない。パイプラインのlayoutはディスクリプタセットの送信先を
/// 指定するために必要。
pub(crate) struct ジオメトリ入力 {
    pub(crate) 頂点バッファ: vk::Buffer,
    pub(crate) インデックスバッファ: vk::Buffer,
    pub(crate) インデックス数: u32,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
}

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
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
    描画方式: 描画方式,
    取得セマフォ: vk::Semaphore,
    提示セマフォ: vk::Semaphore,
    描画完了フェンス: vk::Fence,
) -> Result<bool, レンダラーエラー> {
    let 読み戻し待機が必要 = matches!(描画方式, 描画方式::読み戻し { .. });
    record::コマンドを記録する(
        device,
        command_buffer,
        画像,
        画像ビュー,
        深度画像,
        深度画像ビュー,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
        &描画方式,
    )?;
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
        読み戻し待機が必要,
    )
}
