//! シャドウパスの宣言(判断35): 深度のみのグラフィックスパス。頂点シェーダーが
//! 位置をライトビュー射影変換し、フラグメントは空(色を書かない)。
//! シーン描画と同じ頂点/インデックスバッファ・ディスクリプタセットを、
//! シャドウ専用パイプラインで束ね直して描画する。

use ash::vk;

use super::scene_pass::布ドロー;
use crate::vulkan::frame::シャドウ描画入力;
use crate::vulkan::graph::{
    クリア指定, バッファハンドル, バッファ用途, パス宣言, パス種別, 画像ハンドル, 画像用途
};
use crate::vulkan::shadow_map::シャドウマップ一辺;

pub(super) fn 作る<'a>(
    シャドウマップ: 画像ハンドル,
    スキン済み頂点: Option<バッファハンドル>,
    布ドロー: Option<布ドロー<'a>>,
    入力: &'a シャドウ描画入力,
) -> パス宣言<'a> {
    // 注意: クリア指定のカラー値はカラーアタッチメントを持たないため使われない
    // (深度のみのパス。rendering_setupはカラーハンドルがNoneならこの値を読まない)。
    // 0.0は常に正規化範囲内のため、このgenerate呼び出しが失敗することはない。
    let ダミークリア色 = crate::clear_color::クリアカラー::生成する(0.0, 0.0, 0.0, 0.0)
        .unwrap_or_else(|誤り| panic!("シャドウパスのダミークリア色生成が失敗した(実装のバグ): {誤り}"));
    // スキン付きシーンでは頂点バッファがスキニングパスの出力のため、依存を読み宣言で表す(判断44)。
    // 布があれば布頂点への依存も同様に宣言する(判断54: 布もシャドウを落とす)。
    let mut 読みバッファ一覧 = スキン済み頂点.map_or(Vec::new(), |ハンドル| vec![(ハンドル, バッファ用途::頂点読み)]);
    if let Some(布) = &布ドロー {
        読みバッファ一覧.push((布.頂点ハンドル, バッファ用途::頂点読み));
    }

    パス宣言::生成する(
        "シャドウ",
        Vec::new(),
        vec![(シャドウマップ, 画像用途::深度出力)],
        読みバッファ一覧,
        Vec::new(),
        パス種別::グラフィックス {
            カラー: None,
            深度: Some(シャドウマップ),
            クリア指定: クリア指定::クリアする {
                カラー: ダミークリア色
            },
        },
        move |文脈| {
            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let 一辺 =
                f32::from(u16::try_from(シャドウマップ一辺).unwrap_or_else(|_| panic!("シャドウマップ一辺がu16に収まらない: {シャドウマップ一辺}")));
            let viewport = vk::Viewport::default()
                .x(0.0)
                .y(0.0)
                .width(一辺)
                .height(一辺)
                .min_depth(0.0)
                .max_depth(1.0);
            let シザー = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: シャドウマップ一辺,
                    height: シャドウマップ一辺,
                },
            };
            let viewport一覧 = [viewport];
            let シザー一覧 = [シザー];
            let 頂点バッファ一覧 = [入力.頂点バッファ];
            let オフセット一覧 = [0u64];
            let ディスクリプタセット一覧 = [入力.ディスクリプタセット];

            // 安全性: command_bufferは記録中で、pipeline・各バッファ・ディスクリプタセットは生成済み。
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
                device.cmd_bind_vertex_buffers(command_buffer, 0, &頂点バッファ一覧, &オフセット一覧);
                device.cmd_bind_index_buffer(command_buffer, 入力.インデックスバッファ, 0, vk::IndexType::UINT32);
                device.cmd_draw_indexed(command_buffer, 入力.インデックス数, 1, 0, 0, 0);
                // 布の第2ドロー(判断54)。布頂点は48バイト互換レイアウトのため同じシャドウパイプラインで描ける。
                if let Some(布) = &布ドロー {
                    let 布頂点一覧 = [布.入力.布頂点バッファ];
                    device.cmd_bind_vertex_buffers(command_buffer, 0, &布頂点一覧, &オフセット一覧);
                    device.cmd_bind_index_buffer(command_buffer, 布.入力.インデックスバッファ, 0, vk::IndexType::UINT32);
                    device.cmd_draw_indexed(command_buffer, 布.入力.インデックス数, 1, 0, 0, 0);
                }
            }
        },
    )
}
