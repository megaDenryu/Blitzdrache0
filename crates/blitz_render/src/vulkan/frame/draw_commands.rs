//! パイプラインのバインドと動的ビューポート/シザー設定、プッシュ定数（ビュー射影行列）、
//! 頂点/インデックスバッファのバインドとインデックス描画。

use ash::vk;

use super::ジオメトリ入力;

pub(super) fn 描画コマンドを積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    pipeline: vk::Pipeline,
    寸法: vk::Extent2D,
    ジオメトリ入力: &ジオメトリ入力,
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
    let 頂点バッファ一覧 = [ジオメトリ入力.頂点バッファ];
    let オフセット一覧 = [0u64];
    let 行列バイト列 = 行列をバイト列にする(ジオメトリ入力.ビュー射影行列);

    let ディスクリプタセット一覧 = [ジオメトリ入力.ディスクリプタセット];

    // 安全性: command_bufferは記録中で、pipeline・各バッファ・ディスクリプタセットは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
        device.cmd_push_constants(
            command_buffer,
            ジオメトリ入力.layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            &行列バイト列,
        );
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            ジオメトリ入力.layout,
            0,
            &ディスクリプタセット一覧,
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &頂点バッファ一覧, &オフセット一覧);
        device.cmd_bind_index_buffer(command_buffer, ジオメトリ入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, ジオメトリ入力.インデックス数, 1, 0, 0, 0);
    }
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をVkViewport用のf32へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける。
fn u32を丸めずf32へ変換する(値: u32) -> f32 {
    let 値u16 =
        u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"));
    f32::from(値u16)
}

/// 列優先4x4行列をプッシュ定数用の64バイト列へ変換する。
fn 行列をバイト列にする(行列: &[[f32; 4]; 4]) -> [u8; 64] {
    let mut バイト列 = [0u8; 64];
    for (列添字, 列) in 行列.iter().enumerate() {
        for (成分添字, 成分) in 列.iter().enumerate() {
            let 開始 = (列添字 * 4 + 成分添字) * 4;
            バイト列[開始..開始 + 4].copy_from_slice(&成分.to_le_bytes());
        }
    }
    バイト列
}
