//! 遠方環境の立方体画像の全層をホスト可視バッファへ写す転送パスの宣言。担当する工程は「画像ハンドルと1層の範囲と
//! 層数と受けバッファを受け取り、転送種別のパス宣言を返す」ことである。
//!
//! 大気のベイク済み画像のコピーと別の宣言にするのは、こちらが層数を1でなく6で写すためである。
//! 1回のコピーで全層を写すと、受けバッファの並びは層番号が最も外・縦が次・横が最も速い順になり、
//! CPU正本の`遠方環境を焼く`が返す並びと一致する。

use ash::vk;

use crate::vulkan::graph;

pub(super) fn 作る(
    画像: graph::画像ハンドル, 一層の範囲: vk::Extent3D, 層数: u32, 受け: vk::Buffer
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        "遠方環境読み戻し",
        vec![(画像, graph::画像用途::転送元)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 画像ハンドル = 文脈.画像を解決する(画像);
            let 領域 = [vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .mip_level(0)
                        .base_array_layer(0)
                        .layer_count(層数),
                )
                .image_extent(一層の範囲)];
            // 安全性: command_bufferは記録中、画像はTRANSFER_SRC_OPTIMALへ遷移済み(用途宣言からグラフが導く)、
            // 受けバッファは全層ぶんのテクセル数の容量で確保済みである。
            unsafe {
                文脈
                    .device()
                    .cmd_copy_image_to_buffer(文脈.コマンドバッファ(), 画像ハンドル, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, 受け, &領域);
            }
        },
    )
}
