//! シャドウパスの複数対象と布の描画コマンド記録。

use ash::vk;

use super::scene_pass::布ドロー;
use crate::vulkan::frame::シャドウ描画入力;
use crate::vulkan::relative_anchor;
use crate::vulkan::shadow_map::シャドウマップ一辺;

pub(super) fn 記録する(
    device: &ash::Device, command_buffer: vk::CommandBuffer, 入力一覧: &[シャドウ描画入力], 布ドロー: Option<布ドロー<'_>>
) {
    let Some(pipeline) = 束縛するパイプラインを選ぶ(入力一覧, 布ドロー.as_ref()) else {
        // 影を落とす対象も布も無いフレーム。全個体がライト視錐台の外にある状態で実際に起こる。
        // パスそのものは通してシャドウマップを消去する。消去しないと前フレームの深度が影として残る。
        return;
    };
    let 一辺 = f32::from(u16::try_from(シャドウマップ一辺).unwrap_or_else(|_| panic!("シャドウマップ一辺がu16に収まらない: {シャドウマップ一辺}")));
    let viewport = vk::Viewport::default().width(一辺).height(一辺).min_depth(0.0).max_depth(1.0);
    let シザー = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: vk::Extent2D {
            width: シャドウマップ一辺,
            height: シャドウマップ一辺,
        },
    };
    // 安全性: command_bufferは記録中で、全入力のパイプライン・バッファ・ディスクリプタセットは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);
        device.cmd_set_scissor(command_buffer, 0, &[シザー]);
        for 入力 in 入力一覧 {
            対象を記録する(device, command_buffer, 入力);
        }
        if let Some(布) = 布ドロー {
            布を記録する(device, command_buffer, 布);
        }
    }
}

/// このパスが束縛するシャドウパイプライン。通常の描画対象も布も、レンダラーが1つだけ持つシャドウパイプラインを指すため、
/// 先に得られた方を束縛すれば両方を描ける。どちらも無ければ`None`を返し、パスはシャドウマップの消去だけを行う。
fn 束縛するパイプラインを選ぶ(入力一覧: &[シャドウ描画入力], 布: Option<&布ドロー<'_>>) -> Option<vk::Pipeline> {
    入力一覧
        .first()
        .map(|入力| 入力.pipeline)
        .or_else(|| 布.map(|布| 布.入力.外部資源.シャドウpipeline))
}

unsafe fn 対象を記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, 入力: &シャドウ描画入力) {
    // 安全性: 呼び出し元がコマンド記録中と全入力資源の有効性を保証する。
    unsafe {
        relative_anchor::積む(device, command_buffer, 入力.layout, 入力.相対アンカー);
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            入力.layout,
            0,
            &[入力.ディスクリプタセット],
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[入力.頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 入力.インデックス数, 入力.インスタンス数, 0, 0, 入力.先頭インスタンス);
    }
}

unsafe fn 布を記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, 布: 布ドロー<'_>) {
    let 資源 = &布.入力.外部資源;
    // 安全性: 呼び出し元がコマンド記録中と全入力資源の有効性を保証する。
    unsafe {
        relative_anchor::積む(device, command_buffer, 資源.シャドウlayout, 布.入力.相対アンカー);
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            資源.シャドウlayout,
            0,
            &[資源.シーンディスクリプタセット],
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[布.入力.布頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 布.入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 布.入力.インデックス数, 1, 0, 0, 0);
    }
}
