//! シーン描画パスの宣言。カラー・深度アタッチメントへ書き、記録クロージャは
//! パイプラインのバインドとドローだけを行う(実行器がbegin/end renderingを担う)。

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{draw_commands, shared_set_bind, ジオメトリ入力, 共有セット束縛, 布描画入力};
use crate::vulkan::graph::{
    クリア指定, バッファハンドル, バッファ用途, パス宣言, パス種別, 深度アタッチメント, 画像ハンドル, 画像用途
};
use crate::vulkan::relative_anchor;

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
    共有: 共有セット束縛,
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
            深度: Some(深度アタッチメント::全体(深度)),
            クリア指定: クリア指定::クリアする { カラー: クリア色 },
        },
        move |文脈| {
            draw_commands::描画コマンドを積む(文脈.device(), 文脈.コマンドバッファ(), pipeline, 寸法, ジオメトリ一覧, 共有);
            if let Some(布) = &布ドロー {
                布を記録する(文脈.device(), 文脈.コマンドバッファ(), 布, 共有);
            }
        },
    )
}

/// 布はカメラ視錐台で通常の描画対象が1件も残らないフレームにも描くため、束縛先をジオメトリ一覧の先頭から借りず、
/// 布自身のパイプラインのレイアウトと共有のセットから取る。布のパイプラインレイアウトはset1とset2を空のレイアウトで
/// 宣言するため、通常の描画対象が束縛したset0とset3は無効になっており、ここで結び直す必要がある。
/// 材質のセットは布が読まないため結ばない(空のレイアウトの位置へ実レイアウトのセットを結ぶと互換でない)。
fn 布を記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, 布: &布ドロー<'_>, 共有: 共有セット束縛) {
    let 入力 = 布.入力;
    // 安全性: command_bufferは記録中で、布のパイプライン・バッファは生成済み。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, 入力.描画pipeline);
        relative_anchor::積む(device, command_buffer, 入力.描画layout, 入力.相対の基準原点);
    }
    shared_set_bind::布の共有セットを束縛する(device, command_buffer, 入力.描画layout, 共有);
    // 安全性: command_bufferは記録中で、布の頂点・インデックスバッファは生成済み。
    unsafe {
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[入力.布頂点バッファ], &[0]);
        device.cmd_bind_index_buffer(command_buffer, 入力.インデックスバッファ, 0, vk::IndexType::UINT32);
        device.cmd_draw_indexed(command_buffer, 入力.インデックス数, 1, 0, 0, 0);
    }
}
