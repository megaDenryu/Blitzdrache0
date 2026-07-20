//! コマンド記録: レンダーグラフを構築して実行するだけにする。バリア発行・
//! begin/end renderingはグラフ実行器に集約する（参照: `_doc/設計/レンダーグラフ.md`、
//! `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`判断27〜28）。

mod readback_pass;
mod scene_pass;

use ash::vk;

use super::{ジオメトリ入力, 描画方式};
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::vulkan::graph;

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
    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferはRESET_COMMAND_BUFFERフラグ付きプール由来で、
    // ここでの開始が暗黙的に前回の記録をリセットする。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };

    let グラフ = グラフを構築する(
        画像,
        画像ビュー,
        深度画像,
        深度画像ビュー,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
        描画方式,
    );
    graph::実行する(device, command_buffer, グラフ);

    // 安全性: command_bufferは記録開始済みで、対応するend呼び出し。
    unsafe { device.end_command_buffer(command_buffer)? };
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn グラフを構築する<'a>(
    画像: vk::Image,
    画像ビュー: vk::ImageView,
    深度画像: vk::Image,
    深度画像ビュー: vk::ImageView,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &'a ジオメトリ入力,
    描画方式: &'a 描画方式,
) -> graph::グラフ<'a> {
    let mut グラフ = graph::グラフ::新規(寸法);
    let カラーハンドル = グラフ.画像を登録する(
        画像,
        画像ビュー,
        graph::画像アスペクト::カラー,
        graph::取得直後の色画像状態(),
    );
    let 深度ハンドル = グラフ.画像を登録する(
        深度画像,
        深度画像ビュー,
        graph::画像アスペクト::深度,
        graph::前フレーム深度書き込み直後状態(),
    );

    グラフ.パスを積む(scene_pass::作る(
        カラーハンドル,
        深度ハンドル,
        クリア色,
        pipeline,
        ジオメトリ入力,
        寸法,
    ));

    if let 描画方式::読み戻し { バッファ } = 描画方式 {
        グラフ.パスを積む(readback_pass::作る(カラーハンドル, *バッファ, 寸法));
    }

    グラフ.最終用途を宣言する(カラーハンドル, graph::画像用途::提示);
    グラフ
}
