//! シーン描画パスの宣言。カラー・深度アタッチメントへ書き、記録クロージャは
//! パイプラインのバインドとドローだけを行う(実行器がbegin/end renderingを担う)。

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{draw_commands, ジオメトリ入力, 布描画入力};
use crate::vulkan::graph::{
    クリア指定, バッファハンドル, バッファ用途, パス宣言, パス種別, 画像ハンドル, 画像用途
};
use crate::vulkan::relative_anchor::{self, カメラ相対アンカー};

/// シーン/シャドウパス内の布の第2ドロー(判断54)。頂点ハンドルは布頂点生成パスの出力。
#[derive(Clone, Copy)]
pub(super) struct 布ドロー<'a> {
    pub(super) 入力: &'a 布描画入力,
    pub(super) 頂点ハンドル: バッファハンドル,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 作る<'a>(
    カラー: 画像ハンドル,
    深度: 画像ハンドル,
    シャドウマップ: 画像ハンドル,
    スキン済み頂点: Option<バッファハンドル>,
    布ドロー: Option<布ドロー<'a>>,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ一覧: &'a [ジオメトリ入力],
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    // スキン付きシーンでは頂点バッファがスキニングパスの出力のため、依存を読み宣言で表す(判断44)。
    // 布があれば布頂点(布頂点生成パスの出力)への依存も同様に宣言する(判断54)。
    let mut 読みバッファ一覧ローカル = スキン済み頂点.map_or(Vec::new(), |ハンドル| vec![(ハンドル, バッファ用途::頂点読み)]);
    if let Some(布) = &布ドロー {
        読みバッファ一覧ローカル.push((布.頂点ハンドル, バッファ用途::頂点読み));
    }
    パス宣言::生成する(
        "シーン描画",
        vec![(シャドウマップ, 画像用途::深度シェーダー読み)],
        vec![(カラー, 画像用途::カラー出力), (深度, 画像用途::深度出力)],
        読みバッファ一覧ローカル,
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(カラー),
            深度: Some(深度),
            クリア指定: クリア指定::クリアする { カラー: クリア色 },
        },
        move |文脈| {
            draw_commands::描画コマンドを積む(文脈.device(), 文脈.コマンドバッファ(), pipeline, 寸法, ジオメトリ一覧);
            if let Some(布) = &布ドロー {
                布を記録する(文脈.device(), 文脈.コマンドバッファ(), ジオメトリ一覧, 布);
            }
        },
    )
}

fn 布を記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, 一覧: &[ジオメトリ入力], 布: &布ドロー<'_>) {
    let 先頭 = 一覧.first().unwrap_or_else(|| panic!("シーン描画入力は1件以上の不変条件に違反した"));
    // 安全性: command_bufferは記録中で、布のパイプライン・バッファは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 布.入力.描画pipeline);
        relative_anchor::積む(device, command_buffer, 先頭.layout, カメラ相対アンカー::加算なし());
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            先頭.layout,
            0,
            &[先頭.ディスクリプタセット],
            &[],
        );
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[布.入力.布頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 布.入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 布.入力.インデックス数, 1, 0, 0, 0);
    }
}
