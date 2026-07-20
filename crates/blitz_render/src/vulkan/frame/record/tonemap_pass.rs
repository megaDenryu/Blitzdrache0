//! トーンマップパスの宣言(判断38・39): HDR画像をフラグメントで読み、全画面三角形で
//! スワップチェーンへ露出+ACESの結果を書く。記録クロージャはバインド・プッシュ定数・
//! 3頂点ドローのみを行う(begin/end renderingは実行器が担う)。

use ash::vk;

use crate::vulkan::frame::トーンマップ描画入力;
use crate::vulkan::graph::{クリア指定, パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    hdr: 画像ハンドル,
    スワップチェーン: 画像ハンドル,
    入力: &'a トーンマップ描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    // 注意: 全画面三角形が全ピクセルを上書きするためクリア値は表示に影響しない。
    // クリア指定enumがロード(内容未定義のスワップチェーンでは不可)とクリアの2択のため、黒でクリアする。
    let 黒 = crate::clear_color::クリアカラー::生成する(0.0, 0.0, 0.0, 1.0)
        .unwrap_or_else(|誤り| panic!("トーンマップパスのクリア色生成が失敗した(実装のバグ): {誤り}"));

    パス宣言::生成する(
        "トーンマップ",
        vec![(hdr, 画像用途::シェーダー読みフラグメント)],
        vec![(スワップチェーン, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(スワップチェーン),
            深度: None,
            クリア指定: クリア指定::クリアする { カラー: 黒 },
        },
        move |文脈| {
            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let viewport = vk::Viewport::default()
                .x(0.0)
                .y(0.0)
                .width(crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する(寸法.width))
                .height(crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する(寸法.height))
                .min_depth(0.0)
                .max_depth(1.0);
            let シザー = vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: 寸法 };
            let viewport一覧 = [viewport];
            let シザー一覧 = [シザー];
            let ディスクリプタセット一覧 = [入力.ディスクリプタセット];
            let 露出バイト列 = 入力.露出.to_le_bytes();

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            // プッシュ定数の範囲(FRAGMENT・4バイト)はパイプラインlayoutの宣言と一致する。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.pipeline);
                device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
                device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    入力.layout,
                    0,
                    &ディスクリプタセット一覧,
                    &[],
                );
                device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::FRAGMENT, 0, &露出バイト列);
                device.cmd_draw(command_buffer, 3, 1, 0, 0);
            }
        },
    )
}
