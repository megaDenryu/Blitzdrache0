//! 粒子描画パスの宣言(判断29): シーンと同じカラー/深度アタッチメントへ追記描画する
//! (loadOpはLOAD側、`クリア指定::ロードする`)。頂点入力を持たず、SV_VertexIDで
//! ストレージバッファから位置を読む(バッファ用途::頂点段シェーダー読み)。

use ash::vk;

use crate::vulkan::frame::粒子描画入力;
use crate::vulkan::graph::{
    カラー添付列, クリア指定, バッファハンドル, バッファ用途, パス宣言, パス種別, 深度アタッチメント, 画像ハンドル, 画像用途,
};
use crate::vulkan::relative_anchor;

pub(super) fn 粒子描画パスを宣言する<'a>(
    カラー: 画像ハンドル,
    深度: 画像ハンドル,
    粒子ハンドル: バッファハンドル,
    粒子入力: &'a 粒子描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "粒子描画",
        Vec::new(),
        vec![(カラー, 画像用途::カラー出力), (深度, 画像用途::深度出力)],
        vec![(粒子ハンドル, バッファ用途::頂点段シェーダー読み)],
        Vec::new(),
        パス種別::グラフィックス {
            カラー: カラー添付列::色だけ(カラー),
            深度: Some(深度アタッチメント::全体(深度)),
            クリア指定: クリア指定::ロードする,
        },
        move |文脈| {
            let 積み先 = 文脈.積み先();
            let device = 積み先.論理デバイス();
            let command_buffer = 積み先.コマンドバッファ();
            let set一覧 = [粒子入力.ディスクリプタセット];
            let viewport = vk::Viewport::default()
                .x(0.0)
                .y(0.0)
                .width(寸法幅をf32へ変換する(寸法.width))
                .height(寸法幅をf32へ変換する(寸法.height))
                .min_depth(0.0)
                .max_depth(1.0);
            let シザー = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: 寸法,
            };
            let viewport一覧 = [viewport];
            let シザー一覧 = [シザー];

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 粒子入力.描画パイプライン);
                device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
                device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
                relative_anchor::積む(積み先, 粒子入力.描画layout, 粒子入力.相対の基準原点);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, 粒子入力.描画layout, 0, &set一覧, &[]);
                device.cmd_draw(command_buffer, 粒子入力.描画要素数, 1, 0, 0);
            }
        },
    )
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をVkViewport用のf32へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける
/// (`vulkan::frame::draw_commands`と同じ変換方針)。
fn 寸法幅をf32へ変換する(値: u32) -> f32 {
    let 値u16 = u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"));
    f32::from(値u16)
}
