//! 空パスの宣言: シーンが書いたカラーと深度をLOADし、深度がクリア値のままの画素だけへ全画面三角形で空を描く。
//! 深度は読むだけで書かないため、後続の粒子パスが見る深度はシーンパスの結果のまま変わらない。
//! パイプライン側の深度比較(EQUAL)が画素の限定を担い、このパスは宣言と発行だけを持つ
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの配置」)。

use ash::vk;

use crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する;
use crate::vulkan::frame::空描画入力;
use crate::vulkan::graph::{クリア指定, パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    カラー: 画像ハンドル, 深度: 画像ハンドル, 入力: &'a 空描画入力, 寸法: vk::Extent2D
) -> パス宣言<'a> {
    パス宣言::生成する(
        "空",
        Vec::new(),
        vec![(カラー, 画像用途::カラー出力), (深度, 画像用途::深度出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(カラー),
            深度: Some(深度),
            クリア指定: クリア指定::ロードする,
        },
        move |文脈| {
            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
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
            let セット一覧 = [入力.ディスクリプタセット];
            // 安全性: command_bufferは記録中で、pipelineとディスクリプタセットは互換の組として生成済み。
            // 同期とレイアウト遷移はグラフ実行器が担う。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.pipeline);
                device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
                device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.layout, 0, &セット一覧, &[]);
                device.cmd_draw(command_buffer, 3, 1, 0, 0);
            }
        },
    )
}
