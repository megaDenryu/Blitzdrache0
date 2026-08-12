//! 深度プリパスの描画コマンドの記録。受け取るのは色パスと同じ描画発行の並び、積むのは同じ順・同じ個体数の深度だけの描画である。
//!
//! 色パスと同じ並びをそのまま描くのが要点である。並びを絞ったり並べ替えたりすると、色パスの深度比較を等値にしたときに、
//! プリパスが書かなかった画素の色が丸ごと落ちる。束縛するのはビューとパスのセット(set0)とジオメトリのセット(set1)だけであり、
//! 材質のセットと照明問い合わせのセットは結ばない。画素段が無いため、この段はどちらのセットも1つも読まない。
//!
//! 注意: 描画切替の計器へ数えない。計器はパスごとの発行数と切替数を対応させて報告するため、ここで数えるとシーンの発行数に対して
//! 切替数だけが倍になり、既存の報告の意味が変わる。プリパスの費用はパス別GPU計測が別の区間として持つ。

use ash::vk;

use super::draw_commands::u32を丸めずf32へ変換する;
use super::{shared_set_bind, ジオメトリ入力, 共有セット束縛};
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
        .width(u32を丸めずf32へ変換する(寸法.width))
        .height(u32を丸めずf32へ変換する(寸法.height))
        .min_depth(0.0)
        .max_depth(1.0);
    let シザー = vk::Rect2D::default().extent(寸法);
    // 安全性: command_bufferは記録中で、pipelineと全対象のバッファ・ディスクリプタセットは生成済み。
    unsafe {
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);
        device.cmd_set_scissor(command_buffer, 0, &[シザー]);
    }
    let Some(先頭) = ジオメトリ一覧.first() else {
        return;
    };
    shared_set_bind::ビューとパスのセットを束縛する(積み先, 先頭.layout, 共有);
    let mut 直前のpipeline = None;
    for 入力 in ジオメトリ一覧 {
        if 直前のpipeline != Some(入力.深度プリパスpipeline) {
            // 安全性: command_bufferは記録中で、pipelineは台帳が起動時に作った実体である。
            unsafe { device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.深度プリパスpipeline) };
            直前のpipeline = Some(入力.深度プリパスpipeline);
        }
        一件を記録する(積み先, 入力);
    }
}

/// プッシュ定数も色パスと同じ値を積む。頂点段が読むのは基準原点だけであるが、値を変えると同じ頂点段でも位置が変わりうる。
fn 一件を記録する(積み先: GPU命令の積み先<'_>, 入力: &ジオメトリ入力) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    // 安全性: command_bufferは記録中で、入力のバッファとディスクリプタセットは生成済み。
    unsafe {
        入力.描画定数.プッシュ定数として積む(積み先, 入力.layout);
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
