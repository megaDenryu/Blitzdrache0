//! パイプラインのバインドと動的ビューポート/シザー設定、
//! 頂点/インデックスバッファのバインドとインデックス描画。1回の発行で描く個体の数は入力が持ち、群×段×プリミティブごとに1回だけ発行する
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」、`_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」)。
//! ビュー射影行列等はビューとパスのセット(set0)経由で渡す(判断24)。
//! 描画ごとに変わるカメラ相対の基準原点と材質レコード添字だけをプッシュ定数で積む(参照: `vulkan::scene_draw_constants`)。
//!
//! 注意: set0とset2とset3はパイプラインを束縛した直後に1回だけ結び、発行ごとにはset1だけを結ぶ。
//! 発行の数で増える束縛をジオメトリのセット1つに限ることが、材質/プリミティブ数に比例した束縛を作らない根拠である。

use ash::vk;

use super::shared_set_bind;
use super::{ジオメトリ入力, 共有セット束縛};
use crate::vulkan::scene_draw_constants;

pub(super) fn 描画コマンドを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    pipeline: vk::Pipeline,
    寸法: vk::Extent2D,
    ジオメトリ一覧: &[ジオメトリ入力],
    共有: 共有セット束縛,
) {
    let viewport = vk::Viewport::default()
        .x(0.0)
        .y(0.0)
        .width(u32を丸めずf32へ変換する(寸法.width))
        .height(u32を丸めずf32へ変換する(寸法.height))
        .min_depth(0.0)
        .max_depth(1.0);
    let シザー = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: 寸法,
    };
    let viewport一覧 = [viewport];
    let シザー一覧 = [シザー];
    let 開始位置一覧 = [0u64];

    // 安全性: command_bufferは記録中で、pipelineと全対象のバッファ・ディスクリプタセットは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
        if let Some(先頭) = ジオメトリ一覧.first() {
            shared_set_bind::シーンの共有セットを束縛する(device, command_buffer, 先頭.layout, 共有);
        }
        for 入力 in ジオメトリ一覧 {
            scene_draw_constants::積む(device, command_buffer, 入力.layout, 入力.描画定数);
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                入力.layout,
                shared_set_bind::ジオメトリのセット番号,
                &[入力.ジオメトリセット],
                &[],
            );
            device.cmd_bind_vertex_buffers(command_buffer, 0, &[入力.頂点バッファ], &開始位置一覧);
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
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をVkViewport用のf32へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける。
pub(crate) fn u32を丸めずf32へ変換する(値: u32) -> f32 {
    let 値u16 = u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"));
    f32::from(値u16)
}
