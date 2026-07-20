//! シーン描画パスの宣言。カラー・深度アタッチメントへ書き、記録クロージャは
//! パイプラインのバインドとドローだけを行う(実行器がbegin/end renderingを担う)。

use ash::vk;

use crate::clear_color::クリアカラー;
use crate::vulkan::frame::{draw_commands, ジオメトリ入力};
use crate::vulkan::graph::{クリア指定, パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    カラー: 画像ハンドル,
    深度: 画像ハンドル,
    クリア色: クリアカラー,
    pipeline: vk::Pipeline,
    ジオメトリ入力: &'a ジオメトリ入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "シーン描画",
        Vec::new(),
        vec![(カラー, 画像用途::カラー出力), (深度, 画像用途::深度出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー,
            深度: Some(深度),
            クリア指定: クリア指定::クリアする { カラー: クリア色 },
        },
        move |文脈| {
            draw_commands::描画コマンドを積む(
                文脈.device(),
                文脈.コマンドバッファ(),
                pipeline,
                寸法,
                ジオメトリ入力,
            );
        },
    )
}
