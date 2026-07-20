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

/// GPU粒子トイ(判断29)1フレームぶんの入力。`--particles`指定時のみ`Some`で渡す。
/// 呼び出し元(renderer層)がフレーム添字に対応するディスクリプタセットを
/// あらかじめ選んで渡す(`ジオメトリ入力`と同じ設計)。
pub(crate) struct 粒子描画入力 {
    pub(crate) コンピュートパイプライン: vk::Pipeline,
    pub(crate) コンピュートlayout: vk::PipelineLayout,
    pub(crate) 描画パイプライン: vk::Pipeline,
    pub(crate) 描画layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) バッファ: vk::Buffer,
}

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
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
    粒子入力: Option<&粒子描画入力>,
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
        画像,
        画像ビュー,
        深度画像,
        深度画像ビュー,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
        粒子入力,
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
