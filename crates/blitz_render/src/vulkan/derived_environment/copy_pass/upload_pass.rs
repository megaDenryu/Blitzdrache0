//! 検査が与える遠方環境の中身をホスト可視バッファから立方体画像へ写す転送パスの宣言。担当する工程は
//! 「画像ハンドルと1層の範囲と層数と元バッファを受け取り、転送種別のパス宣言を返す」ことである。
//!
//! 読み戻しの向きと別の宣言にするのは、用途が転送先であり、グラフが導くレイアウトも反対だからである。

use ash::vk;

use super::層の部分範囲;
use crate::vulkan::graph;

pub(in crate::vulkan) fn 書き込みパスを作る(
    画像: graph::画像ハンドル,
    範囲: vk::Extent3D,
    層数: u32,
    元: vk::Buffer,
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        "検査用遠方環境の書き込み",
        Vec::new(),
        vec![(画像, graph::画像用途::転送先)],
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 画像ハンドル = 文脈.画像を解決する(画像);
            let 領域 = [vk::BufferImageCopy::default().image_subresource(層の部分範囲(0, 層数)).image_extent(範囲)];
            // 安全性: command_bufferは記録中、画像はTRANSFER_DST_OPTIMALへ遷移済み(用途宣言からグラフが導く)、
            // 元バッファは全層ぶんのテクセル数の容量で確保し中身を書き終えている。
            unsafe {
                文脈
                    .device()
                    .cmd_copy_buffer_to_image(文脈.コマンドバッファ(), 元, 画像ハンドル, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &領域);
            }
        },
    )
}
