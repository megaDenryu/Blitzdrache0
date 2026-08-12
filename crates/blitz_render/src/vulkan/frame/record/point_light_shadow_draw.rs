//! 点光源の影の1つの面ぶんの描画コマンドの記録。担うのは、その面の候補を絞ることと、絞って残った発行を積むことである。
//!
//! 絞りを記録の地点で行うのは、面ごとの候補を可視ID列の区間として持たない裁定によるものである。候補は
//! そのフレームの記録の中だけで使い、どこにも残さない(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断n」)。
//! 数えるのもここである。候補の数と実際に発行した数は、絞った地点でしか正しく数えられない。

use ash::vk;

use crate::cube_image::立方体の面;
use crate::point_light_shadow::点光源の影の面の一辺;
use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::frame::shared_set_bind;
use crate::vulkan::frame::{共有セット束縛, 点光源の影の描画発行, 点光源の影の束縛};
use crate::vulkan::point_light_shadow_cull::面ごとの絞り;
use crate::vulkan::point_light_shadow_plan::影を落とす灯;
use crate::vulkan::point_light_shadow_push;

pub(super) fn 記録する(
    積み先: GPU命令の積み先<'_>,
    灯: 影を落とす灯,
    面: 立方体の面,
    束縛: 点光源の影の束縛,
    発行一覧: &[点光源の影の描画発行],
    共有: 共有セット束縛<'_>,
) {
    let 絞り = 面ごとの絞り::生成する(灯.カメラ相対位置, 灯.影響半径, 面);
    let ライトビュー射影 = 灯.投影の契約.面のライトビュー射影を組み立てる(面, 灯.カメラ相対位置);
    共有.計器.点光源の影().面の判定した発行を数える(発行一覧.len());
    // 候補を先に列へ集めずその場で積むのは、毎フレームの確保を面の数だけ作らないためである。
    // その面へ影を落とす対象が1つも無いフレームでは、パスは通るが1つも束縛しない。層の消去はパスの側が行う
    // (消去しないと前フレームの深度が影として残る)。
    let mut 束縛済みか = false;
    for 発行 in 発行一覧.iter().filter(|発行| 発行.候補か(絞り)) {
        共有.計器.点光源の影().候補を数える();
        if !束縛済みか {
            面の状態を束縛する(積み先, 束縛, 共有);
            束縛済みか = true;
        }
        一件を記録する(積み先, 束縛, 発行, ライトビュー射影);
        共有.計器.点光源の影().描画を数える();
    }
}

/// その面の全発行で変わらない状態(ビューポート・シザー・パイプライン・ビューとパスのセット)を1回だけ束縛する。
fn 面の状態を束縛する(積み先: GPU命令の積み先<'_>, 束縛: 点光源の影の束縛, 共有: 共有セット束縛<'_>) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    let 実数の一辺 = f32::from(u16::try_from(点光源の影の面の一辺).unwrap_or_else(|_| panic!("面の一辺がu16に収まらない")));
    let viewport = vk::Viewport::default().width(実数の一辺).height(実数の一辺).min_depth(0.0).max_depth(1.0);
    let シザー = vk::Rect2D::default().extent(vk::Extent2D::default().width(点光源の影の面の一辺).height(点光源の影の面の一辺));
    // 安全性: command_bufferは記録中であり、ビューポートとシザーはこのパイプラインが動的に宣言する。
    unsafe {
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);
        device.cmd_set_scissor(command_buffer, 0, &[シザー]);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 束縛.pipeline);
    }
    shared_set_bind::ビューとパスのセットを束縛する(積み先, 束縛.layout, 共有);
}

fn 一件を記録する(
    積み先: GPU命令の積み先<'_>,
    束縛: 点光源の影の束縛,
    発行: &点光源の影の描画発行,
    ライトビュー射影: blitz_math::変換<blitz_math::ワールド, blitz_math::点光源の面クリップ>,
) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    // 安全性: command_bufferは記録中で、発行のバッファとディスクリプタセットは生成済み。layoutは80バイトの範囲を宣言済み。
    unsafe {
        point_light_shadow_push::積む(積み先, 束縛.layout, 発行.相対の基準原点, ライトビュー射影);
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            束縛.layout,
            shared_set_bind::ジオメトリのセット番号,
            &[発行.ジオメトリセット],
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[発行.頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 発行.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 発行.インデックス数, 発行.個体数, 発行.先頭インデックス, 発行.頂点基準, 0);
    }
}
