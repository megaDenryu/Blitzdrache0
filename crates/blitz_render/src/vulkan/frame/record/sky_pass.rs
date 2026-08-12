//! 空パスの宣言: シーンが書いたカラーと深度をLOADし、深度がクリア値のままの画素だけへ全画面三角形で空を描く。
//! 深度は読むだけで書かないため、後続の粒子パスが見る深度はシーンパスの結果のまま変わらない。
//! パイプライン側の深度比較(EQUAL)が画素の限定を担い、このパスは宣言と発行だけを持つ
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの配置」)。
//!
//! 読み画像はスカイビューのベイク済み画像と透過率のベイク済み画像の2枚である。どちらも画素段で参照するため、呼び出し元が
//! その2枚のハンドルと用途を渡す。

use ash::vk;

use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する;
use crate::vulkan::frame::空描画入力;
use crate::vulkan::graph::{
    カラー添付列, クリア指定, パス宣言, パス種別, 深度アタッチメント, 画像ハンドル, 画像用途
};

pub(super) fn 空パスを宣言する<'a>(
    カラー: 画像ハンドル,
    動きベクトル: 画像ハンドル,
    深度: 画像ハンドル,
    読み画像: Vec<(画像ハンドル, 画像用途)>,
    入力: &'a 空描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "空",
        読み画像,
        vec![
            (カラー, 画像用途::カラー出力),
            (動きベクトル, 画像用途::カラー出力),
            (深度, 画像用途::深度出力),
        ],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: カラー添付列::色と動きベクトル {
                色: カラー, 動きベクトル
            },
            深度: Some(深度アタッチメント::全体(深度)),
            クリア指定: クリア指定::ロードする,
        },
        move |文脈| {
            let 積み先 = 文脈.積み先();
            let device = 積み先.論理デバイス();
            let command_buffer = 積み先.コマンドバッファ();
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
            // 安全性: command_bufferは記録中で、pipelineとディスクリプタセットは互換の組として生成済み。
            // 同期とレイアウト遷移はグラフ実行器が担う。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.pipeline);
                device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
                device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
            }
            セットを束縛する(積み先, 入力);
            // 安全性: 直前にパイプラインとディスクリプタを束縛済みで、頂点は全画面三角形の3頂点である。
            unsafe { device.cmd_draw(command_buffer, 3, 1, 0, 0) };
        },
    )
}

/// set0にシーンのセット、set1にベイク済み画像の標本セットを束縛する。空パイプラインのレイアウトは常にこの2本を持つ。
fn セットを束縛する(積み先: GPU命令の積み先<'_>, 入力: &空描画入力) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    let セット一覧 = [入力.ディスクリプタセット, 入力.標本セット];
    // 安全性: command_bufferは記録中で、layoutとセットは互換の組として生成済みである。
    unsafe {
        device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.layout, 0, &セット一覧, &[]);
    }
}
