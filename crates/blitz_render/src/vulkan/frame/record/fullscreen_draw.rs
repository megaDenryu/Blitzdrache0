//! 全画面三角形パス(トーンマップ・ブルーム)の共通描画コマンド:
//! ビューポート/シザー設定・バインド・任意のプッシュ定数・3頂点ドロー。

use ash::vk;

use crate::vulkan::frame::draw_commands::u32を丸めずf32へ変換する;
use crate::vulkan::graph::クリア指定;

/// 前提: command_bufferは記録中で、pipeline・layout・ディスクリプタセットは互換の組として
/// 生成済み。プッシュ定数を渡す場合、そのバイト数はlayoutのFRAGMENT範囲宣言と一致すること。
pub(super) fn コマンドを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    ディスクリプタセット: vk::DescriptorSet,
    寸法: vk::Extent2D,
    プッシュ定数: Option<&[u8]>,
) {
    let viewport = vk::Viewport::default()
        .x(0.0)
        .y(0.0)
        .width(u32を丸めずf32へ変換する(寸法.width))
        .height(u32を丸めずf32へ変換する(寸法.height))
        .min_depth(0.0)
        .max_depth(1.0);
    let viewport一覧 = [viewport];
    let シザー一覧 = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: 寸法,
    }];
    let ディスクリプタセット一覧 = [ディスクリプタセット];

    // 安全性: 上記前提のとおり。バインドとドローだけで、同期・レイアウト遷移はグラフ実行器が担う。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
        device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, layout, 0, &ディスクリプタセット一覧, &[]);
        if let Some(バイト列) = プッシュ定数 {
            device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::FRAGMENT, 0, バイト列);
        }
        device.cmd_draw(command_buffer, 3, 1, 0, 0);
    }
}

/// 全画面三角形が全ピクセルを上書きするパス用のクリア指定。クリア値は表示に影響しないが、
/// クリア指定enumがロード(内容未定義の画像では不可)とクリアの2択のため黒でクリアする。
pub(super) fn 黒クリア() -> クリア指定 {
    let 黒 = crate::clear_color::クリアカラー::生成する(0.0, 0.0, 0.0, 1.0)
        .unwrap_or_else(|誤り| panic!("全画面パスのクリア色生成が失敗した(実装のバグ): {誤り}"));
    クリア指定::クリアする { カラー: 黒 }
}
