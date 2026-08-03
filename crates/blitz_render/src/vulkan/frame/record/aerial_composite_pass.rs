//! 空中遠近合成パスの宣言: シーンが書いたカラーをLOADし、深度と空中遠近ボリュームを画素段で読んで、
//! 全画面三角形でジオメトリの画素へ霞を混ぜる。混ぜ方はパイプラインのブレンドが担い、このパスは宣言と発行だけを持つ。
//!
//! 深度をアタッチメントとして宣言しないのは、同じパスで深度を読みながら深度アタッチメントにも指定すると
//! フィードバックループになるためである。深度がクリア値のままの画素の除外は画素段の`discard`が担う。
//! ボリュームの用途に`焼いた画像の画素段参照`を使うのは、大気のベイク済み画像が休むレイアウトをGENERALに保つ不変条件が
//! あるためであり、通常の`シェーダー読み画素段`を使ってはならない(参照: `graph/initial_state/atmosphere_lut.rs`)。

use ash::vk;

use crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する;
use crate::vulkan::frame::空中遠近合成描画入力;
use crate::vulkan::graph::{クリア指定, パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    カラー: 画像ハンドル,
    深度: 画像ハンドル,
    空中遠近: 画像ハンドル,
    入力: &'a 空中遠近合成描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "空中遠近合成",
        vec![(深度, 画像用途::深度シェーダー読み), (空中遠近, 画像用途::焼いた画像の画素段参照)],
        vec![(カラー, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(カラー),
            深度: None,
            クリア指定: クリア指定::ロードする,
        },
        move |文脈| コマンドを積む(文脈.device(), 文脈.コマンドバッファ(), 入力, 寸法),
    )
}

fn コマンドを積む(device: &ash::Device, command_buffer: vk::CommandBuffer, 入力: &空中遠近合成描画入力, 寸法: vk::Extent2D) {
    let viewport一覧 = [vk::Viewport::default()
        .x(0.0)
        .y(0.0)
        .width(u32を丸めずf32へ変換する(寸法.width))
        .height(u32を丸めずf32へ変換する(寸法.height))
        .min_depth(0.0)
        .max_depth(1.0)];
    let シザー一覧 = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: 寸法,
    }];
    let セット一覧 = [入力.シーンセット, 入力.合成セット];
    let 最遠距離バイト列 = 入力.最遠距離.to_le_bytes();
    // 安全性: command_bufferは記録中で、pipelineとlayoutとセットは互換の組として生成済み。即時定数の
    // バイト数はlayoutのFRAGMENT範囲宣言(実数1つ)と一致する。同期とレイアウト遷移はグラフ実行器が担う。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
        device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.layout, 0, &セット一覧, &[]);
        device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::FRAGMENT, 0, &最遠距離バイト列);
        device.cmd_draw(command_buffer, 3, 1, 0, 0);
    }
}
