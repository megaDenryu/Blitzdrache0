//! コマンド記録: レイアウト遷移(カラー・深度) → dynamic renderingでの描画 →
//! (通常|読み戻し)の後処理。アタッチメント記述は`render_pass`に委ねる。

mod render_pass;

use ash::vk;

use super::{barrier, copy, readback_barrier, ジオメトリ入力, 描画方式};
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;

#[allow(clippy::too_many_arguments)]
pub(super) fn コマンドを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
    描画方式: &描画方式,
) -> Result<(), レンダラーエラー> {
    let begin_info = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferはRESET_COMMAND_BUFFERフラグ付きプール由来で、
    // ここでの開始が暗黙的に前回の記録をリセットする。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };

    let 部分範囲 = barrier::部分範囲を作る();
    barrier::描画前バリアを積む(device, command_buffer, 画像, 部分範囲);
    let 深度部分範囲 = barrier::深度用部分範囲を作る();
    barrier::深度前バリアを積む(device, command_buffer, 深度画像, 深度部分範囲);

    render_pass::レンダリングを記録する(
        device,
        command_buffer,
        画像ビュー,
        深度画像ビュー,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
    );

    後処理バリアとコピーを積む(device, command_buffer, 画像, 部分範囲, 寸法, 描画方式);

    // 安全性: command_bufferは記録開始済みで、対応するend呼び出し。
    unsafe { device.end_command_buffer(command_buffer)? };
    Ok(())
}

fn 後処理バリアとコピーを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像: vk::Image,
    部分範囲: vk::ImageSubresourceRange,
    寸法: vk::Extent2D,
    描画方式: &描画方式,
) {
    match 描画方式 {
        描画方式::通常 => barrier::提示前バリアを積む(device, command_buffer, 画像, 部分範囲),
        描画方式::読み戻し { バッファ } => {
            readback_barrier::コピー前バリアを積む(device, command_buffer, 画像, 部分範囲);
            copy::コピーを記録する(device, command_buffer, 画像, *バッファ, 寸法);
            readback_barrier::コピー後バリアを積む(device, command_buffer, 画像, 部分範囲);
        }
    }
}
