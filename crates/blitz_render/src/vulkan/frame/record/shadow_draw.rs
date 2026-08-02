//! シャドウパスの複数対象と布の描画コマンド記録。
//! 通常の描画対象と布はパイプラインが別のため、対象の記録をひとまとめに済ませてから布のパイプラインへ切り替える。
//! どちらの局面もビューとパスのセット(set0)を自分のパイプラインレイアウトで結び直す。パイプラインレイアウトが違うと
//! 束縛が無効になるためである。照明問い合わせのセットはシャドウ記録のレイアウトが宣言しないため結ばない。

use ash::vk;

use super::scene_pass::布ドロー;
use crate::cascade::距離区分番号;
use crate::visible_instance_selection::可視パス;
use crate::vulkan::frame::shared_set_bind;
use crate::vulkan::frame::{シャドウ描画入力, 共有セット束縛};
use crate::vulkan::shadow_map::シャドウマップ一辺;
use crate::vulkan::shadow_push;

pub(super) fn 記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    番号: 距離区分番号,
    入力一覧: &[シャドウ描画入力],
    布ドロー: Option<布ドロー<'_>>,
    共有: 共有セット束縛<'_>,
) {
    if 入力一覧.is_empty() && 布ドロー.is_none() {
        // その距離区分へ影を落とす対象も布も無いフレーム。全個体がその距離区分のライト視錐台の外にある状態で実際に起こる。
        // パスそのものは通してシャドウマップを消去する。消去しないと前フレームの深度が影として残る。
        return;
    }
    let 一辺 = f32::from(u16::try_from(シャドウマップ一辺).unwrap_or_else(|_| panic!("シャドウマップ一辺がu16に収まらない: {シャドウマップ一辺}")));
    let viewport = vk::Viewport::default().width(一辺).height(一辺).min_depth(0.0).max_depth(1.0);
    let シザー = vk::Rect2D::default().extent(vk::Extent2D::default().width(シャドウマップ一辺).height(シャドウマップ一辺));
    // 安全性: command_bufferは記録中であり、ビューポートとシザーはどちらのパイプラインも動的宣言のため、パイプラインの切り替えで失われない。
    unsafe {
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);
        device.cmd_set_scissor(command_buffer, 0, &[シザー]);
    }
    let パス = 可視パス::影の距離区分(番号);
    if let Some(先頭) = 入力一覧.first() {
        // 安全性: command_bufferは記録中で、pipelineは生成済み。
        unsafe { device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 先頭.pipeline) };
        共有.計器.描画切替().パイプライン束縛を数える(パス);
        shared_set_bind::ビューとパスのセットを束縛する(device, command_buffer, 先頭.layout, 共有);
        for 入力 in 入力一覧 {
            共有.計器.セット別束縛().数える(shared_set_bind::ジオメトリのセット番号);
            対象を記録する(device, command_buffer, 番号, 入力);
            共有.計器.描画切替().描画を数える(パス);
        }
    }
    if let Some(布) = 布ドロー {
        布を記録する(device, command_buffer, 番号, 布, 共有);
    }
}

fn 対象を記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, 番号: 距離区分番号, 入力: &シャドウ描画入力) {
    // 安全性: command_bufferは記録中で、入力のバッファとディスクリプタセットは生成済み。
    unsafe {
        shadow_push::積む(device, command_buffer, 入力.layout, 入力.相対の基準原点, 番号);
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            入力.layout,
            shared_set_bind::ジオメトリのセット番号,
            &[入力.ジオメトリセット],
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[入力.頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(
            command_buffer,
            入力.インデックス数,
            入力.インスタンス数,
            入力.先頭インデックス,
            入力.頂点基準,
            入力.先頭インスタンス,
        );
    }
}

/// 布は専用のシャドウパイプラインを束縛し、ジオメトリのセットを読まない。パイプラインもセットも描画対象から借りないため、
/// 描画対象が1件も無いフレームでも、走査順が入れ替わったフレームでも、布の影は同じ位置に出る。
fn 布を記録する(
    device: &ash::Device, command_buffer: vk::CommandBuffer, 番号: 距離区分番号, 布: 布ドロー<'_>, 共有: 共有セット束縛<'_>
) {
    let シャドウ = &布.入力.外部資源.シャドウ;
    // 安全性: command_bufferは記録中で、布のパイプラインは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, シャドウ.pipeline);
        shadow_push::積む(device, command_buffer, シャドウ.layout, 布.入力.相対の基準原点, 番号);
    }
    共有.計器.描画切替().パイプライン束縛を数える(可視パス::影の距離区分(番号));
    shared_set_bind::ビューとパスのセットを束縛する(device, command_buffer, シャドウ.layout, 共有);
    // 安全性: command_bufferは記録中で、布の頂点・インデックスバッファは生成済み。
    unsafe {
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[布.入力.布頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 布.入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 布.入力.インデックス数, 1, 0, 0, 0);
    }
    共有.計器.描画切替().描画を数える(可視パス::影の距離区分(番号));
}
