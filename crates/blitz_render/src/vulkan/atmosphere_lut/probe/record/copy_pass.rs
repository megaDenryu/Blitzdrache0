//! LUT画像1枚をホスト可視バッファへ写す転送パスの宣言。担当する工程は「画像ハンドルと寸法と受けバッファを受け取り、
//! 転送種別のパス宣言を返す」ことである。
//!
//! 生バリアでなくパス宣言にするのは、生成の完了とコピーの間の同期をグラフの導出に任せるためである。
//! 検査だけが本番と違う同期の下で通ることを避ける。

use ash::vk;

use crate::vulkan::graph;

pub(super) fn 作る(
    名前: &'static str, 画像: graph::画像ハンドル, 寸法: vk::Extent2D, 受け: vk::Buffer
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        名前,
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
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: 寸法.width,
                    height: 寸法.height,
                    depth: 1,
                })];
            // 安全性: command_bufferは記録中、画像はTRANSFER_SRC_OPTIMALへ遷移済み(用途宣言からグラフが導く)、
            // 受けバッファはテクセル数ぶんの容量で確保済みである。
            unsafe {
                文脈
                    .device()
                    .cmd_copy_image_to_buffer(文脈.コマンドバッファ(), 画像ハンドル, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, 受け, &領域);
            }
        },
    )
}
