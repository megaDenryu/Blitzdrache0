//! パイプラインのバインドと動的ビューポート/シザー設定、
//! 頂点/インデックスバッファのバインドとインデックス描画。
//! ビュー射影行列等はフレームユニフォームバッファ(ディスクリプタセット)経由で
//! 渡す(判断24。プッシュ定数は廃止)。

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
    let ディスクリプタセット一覧 = [ジオメトリ入力.ディスクリプタセット];

    // 安全性: command_bufferは記録中で、pipeline・各バッファ・ディスクリプタセットは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
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
