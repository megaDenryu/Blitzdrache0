//! コマンド記録: レンダーグラフを構築して実行するだけにする。バリア発行・
//! begin/end renderingはグラフ実行器に集約する（参照: `_doc/設計/レンダーグラフ.md`、
//! `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`判断27〜28）。グラフの
//! 組み立て自体は`graph_build`に委ねる。

mod graph_build;
mod particle_draw_pass;
mod particle_update_pass;
mod readback_pass;
mod scene_pass;
mod shadow_pass;
mod tonemap_pass;
mod ui_pass;

use ash::vk;

use super::{シャドウ描画入力, ジオメトリ入力, トーンマップ描画入力, フレーム画像一式, 描画方式, 粒子描画入力, UI描画入力};
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::vulkan::graph;
use crate::vulkan::gpu_timing;

#[allow(clippy::too_many_arguments)]
pub(super) fn コマンドを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像一式: &フレーム画像一式,
    寸法: vk::Extent2D,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &ジオメトリ入力,
    シャドウ入力: &シャドウ描画入力,
    粒子入力: Option<&粒子描画入力>,
    トーンマップ入力: Option<&トーンマップ描画入力>,
    ui入力: Option<&UI描画入力>,
    描画方式: &描画方式,
    クエリプール: Option<vk::QueryPool>,
) -> Result<Vec<(&'static str, u32)>, レンダラーエラー> {
    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 安全性: command_bufferはRESET_COMMAND_BUFFERフラグ付きプール由来で、
    // ここでの開始が暗黙的に前回の記録をリセットする。
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };

    if let Some(pool) = クエリプール {
        // 安全性: command_bufferは記録開始済みで、poolはこのスロット専用に生成済み。
        // このフレームで書くクエリより前に必ずリセットする(未リセットのクエリへの
        // 書き込みはVulkanの契約違反になる)。
        unsafe { device.cmd_reset_query_pool(command_buffer, pool, 0, gpu_timing::パス数上限 * 2) };
    }

    let グラフ = graph_build::グラフを構築する(
        画像一式,
        寸法,
        クリア色,
        pipeline,
        ジオメトリ入力,
        シャドウ入力,
        粒子入力,
        トーンマップ入力,
        ui入力,
        描画方式,
    );
    let 計測マッピング = graph::実行する(device, command_buffer, グラフ, クエリプール);

    // 安全性: command_bufferは記録開始済みで、対応するend呼び出し。
    unsafe { device.end_command_buffer(command_buffer)? };
    Ok(計測マッピング)
}
