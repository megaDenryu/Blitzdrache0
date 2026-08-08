//! 合成深度を深度画像へ書き戻す転送パスの宣言。受け取るのは深度のハンドルと注入の入力、返すのはパス宣言1本である。
//!
//! 注意: 積む位置は深度プリパスの後・局所可視性補正の2本の前である。前へ置くとプリパスが注入を上書きし、
//! 後ろへ置くと遮蔽の標本化が本番のジオメトリの深度を読む。どちらも「与えた形とは違う深度から焼いた可視度」を
//! 正本と突き合わせることになる。積む位置を持つのは`graph_build`の側であり、この宣言は順序を1つも知らない。

use ash::vk;

use super::合成深度の注入入力;
use crate::vulkan::graph::{
    GPU命令の積み先と宣言済み資源の取り出し口, パス宣言, パス種別, 画像ハンドル, 画像用途
};

pub(crate) fn 合成深度の注入を作る<'a>(深度: 画像ハンドル, 入力: 合成深度の注入入力) -> パス宣言<'a> {
    パス宣言::生成する(
        "合成深度の注入",
        Vec::new(),
        vec![(深度, 画像用途::転送先)],
        Vec::new(),
        Vec::new(),
        パス種別::転送,
        move |文脈| コピーを積む(文脈, 深度, 入力),
    )
}

fn コピーを積む(
    文脈: &GPU命令の積み先と宣言済み資源の取り出し口, 深度: 画像ハンドル, 入力: 合成深度の注入入力
) {
    let 画像 = 文脈.画像を解決する(深度);
    let 領域 = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::DEPTH)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: 入力.寸法.width,
            height: 入力.寸法.height,
            depth: 1,
        });
    let 領域一覧 = [領域];
    // 安全性: command_bufferは記録中、深度画像はグラフの導いたバリアでTRANSFER_DST_OPTIMALへ遷移済み、
    // バッファは寸法ぶんの単精度の列で確保済みである(`合成深度の注入一式::生成する`が同じ寸法から作る)。
    unsafe {
        文脈.device().cmd_copy_buffer_to_image(
            文脈.コマンドバッファ(),
            入力.バッファ,
            画像,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &領域一覧,
        );
    }
}
