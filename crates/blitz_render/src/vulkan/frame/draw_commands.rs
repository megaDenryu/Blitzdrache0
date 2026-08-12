//! シーンパスの描画コマンドの記録。動的ビューポート/シザーの設定、発行ごとのパイプラインの切替、
//! 頂点/インデックスバッファの束縛とインデックス描画を行う。1回の発行で描く個体の数は入力が持ち、群×段×プリミティブごとに1回だけ発行する
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」、`_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」)。
//! ビュー射影行列等はビューとパスのセット(set0)経由で渡す(判断24)。
//! 描画ごとに変わるカメラ相対の基準原点と材質レコード添字だけをプッシュ定数で積む(参照: `vulkan::scene_draw_constants`)。
//!
//! 注意: set0とset2とset3はパスの先頭で1回だけ結び、発行ごとにはset1だけを結ぶ。材質を読む描画族のパイプラインはレイアウトを族で1つ共有するため、
//! パイプラインを切り替えてもこの3つの束縛は無効にならない(参照: `vulkan::pipeline_ledger::layouts`)。
//! ビューポートとシザーは発行が1件も無いフレームでも設定する。同じパスの中で布が自分のパイプラインで描くため、設定を発行の有無に依らせると布が未設定のビューポートで描かれる。

use ash::vk;

use super::shared_set_bind;
use super::{ジオメトリ入力, 共有セット束縛};
use crate::visible_instance_selection::可視パス;
use crate::vulkan::command_sink::GPU命令の積み先;

pub(super) fn 描画コマンドを積む(
    積み先: GPU命令の積み先<'_>,
    寸法: vk::Extent2D,
    ジオメトリ一覧: &[ジオメトリ入力],
    共有: 共有セット束縛<'_>,
) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
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
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
        if let Some(先頭) = ジオメトリ一覧.first() {
            shared_set_bind::シーンの共有セットを束縛する(積み先, 先頭.layout, 共有);
        }
        let mut 直前のキー = None;
        for 入力 in ジオメトリ一覧 {
            if 直前のキー != Some(入力.パイプラインキー) {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.pipeline);
                共有.計器.描画切替().パイプライン束縛を数える(可視パス::シーン);
                直前のキー = Some(入力.パイプラインキー);
            }
            共有.計器.描画切替().材質を見る(可視パス::シーン, 入力.大域材質id);
            入力.描画定数.プッシュ定数として積む(積み先, 入力.layout);
            共有.計器.セット別束縛().数える(shared_set_bind::ジオメトリのセット番号);
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
            共有.計器.描画切替().描画を数える(可視パス::シーン);
        }
    }
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をVkViewport用のf32へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける。
pub(crate) fn u32を丸めずf32へ変換する(値: u32) -> f32 {
    let 値u16 = u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"));
    f32::from(値u16)
}
