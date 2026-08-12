//! 画像の中身をホスト可視バッファへ写す転送パスの宣言。担当する工程は「画像ハンドルと段ごとの範囲と受けバッファを
//! 受け取り、転送種別のパス宣言を返す」ことである。立方体の全段を写すものと、2次元の表を写すものの2つを持つ。
//!
//! 立方体の全段を1本のコピーで写すのは、段ごとに寸法が違いバッファの中の開始位置も違うためである。
//! 段ごとの領域を並べると、受けバッファの並びは段が最も外、次に層、縦、横が最も速い順になり、
//! CPU正本が段ごとに焼く並びとそろう。

use ash::vk;

use super::{四成分テクセルのバイト数, 層の部分範囲};
use crate::vulkan::graph;

pub(in crate::vulkan) fn 立方体の読み戻しを作る(
    パス名: &'static str,
    画像: graph::画像ハンドル,
    段ごとの範囲: Vec<vk::Extent3D>,
    層数: u32,
    受け: vk::Buffer,
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        パス名,
        vec![(画像, graph::画像用途::転送元)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 画像ハンドル = 文脈.画像を解決する(画像);
            let 領域一覧 = 段ごとの領域を並べる(&段ごとの範囲, 層数);
            // 安全性: command_bufferは記録中、画像はTRANSFER_SRC_OPTIMALへ遷移済み(用途宣言からグラフが導く)、
            // 受けバッファは全段全層ぶんのテクセル数の容量で確保済みである。
            unsafe {
                文脈.積み先().論理デバイス().cmd_copy_image_to_buffer(
                    文脈.積み先().コマンドバッファ(),
                    画像ハンドル,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    受け,
                    &領域一覧,
                );
            }
        },
    )
}

pub(in crate::vulkan) fn 表の読み戻しを作る(
    画像: graph::画像ハンドル,
    範囲: vk::Extent3D,
    受け: vk::Buffer,
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        "反射率積分表読み戻し",
        vec![(画像, graph::画像用途::転送元)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 画像ハンドル = 文脈.画像を解決する(画像);
            let 領域 = [vk::BufferImageCopy::default().image_subresource(層の部分範囲(0, 1)).image_extent(範囲)];
            // 安全性: command_bufferは記録中、画像はTRANSFER_SRC_OPTIMALへ遷移済み、受けバッファは全テクセルの容量で確保済みである。
            unsafe {
                文脈.積み先().論理デバイス().cmd_copy_image_to_buffer(
                    文脈.積み先().コマンドバッファ(),
                    画像ハンドル,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    受け,
                    &領域,
                );
            }
        },
    )
}

fn 段ごとの領域を並べる(段ごとの範囲: &[vk::Extent3D], 層数: u32) -> Vec<vk::BufferImageCopy> {
    let mut 開始テクセル = 0_u64;
    let mut 領域一覧 = Vec::with_capacity(段ごとの範囲.len());
    for (段, 範囲) in 段ごとの範囲.iter().enumerate() {
        let 段番号 = u32::try_from(段).unwrap_or_else(|_| panic!("縮小段の番号がu32に収まらない"));
        領域一覧.push(
            vk::BufferImageCopy::default()
                .buffer_offset(開始テクセル * 四成分テクセルのバイト数)
                .image_subresource(層の部分範囲(段番号, 層数))
                .image_extent(*範囲),
        );
        開始テクセル += u64::from(範囲.width) * u64::from(範囲.height) * u64::from(層数);
    }
    領域一覧
}
