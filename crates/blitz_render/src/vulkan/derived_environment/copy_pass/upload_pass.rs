//! ホスト可視バッファから画像へ写す転送パスの宣言。担当する工程は「画像ハンドルと段ごとの範囲と層数と
//! 元バッファを受け取り、転送種別のパス宣言を返す」ことである。
//!
//! 読み戻しの向きと別の宣言にするのは、用途が転送先であり、グラフが導くレイアウトも反対だからである。
//!
//! 段の並びを領域の列で表すのは、1回の転送で全段を埋めるためである。Vulkanは1つの領域の中で層を連続に並べる
//! 規約を持つため、元バッファの並びは「段が最も外、次に層」になる。この並びは注入するバイト列の作り手と
//! 一致していなければならない(`vulkan::indirect_lighting::injection::bytes`)。

use ash::vk;

use super::{四成分テクセルのバイト数, 層の部分範囲};
use crate::vulkan::graph;

/// 立方体画像の最詳細段だけを書き込む。検査が与える遠方環境の中身がこの形を取る。
pub(in crate::vulkan) fn 書き込みパスを作る(
    画像: graph::画像ハンドル,
    範囲: vk::Extent3D,
    層数: u32,
    元: vk::Buffer,
) -> graph::パス宣言<'static> {
    転送パスを作る("検査用遠方環境の書き込み", 画像, vec![範囲], 層数, 元, 四成分テクセルのバイト数)
}

/// 立方体画像の全段を書き込む。段ごとの範囲は段番号の昇順で渡す。
pub(in crate::vulkan) fn 全段の書き込みパスを作る(
    パス名: &'static str,
    画像: graph::画像ハンドル,
    段ごとの範囲: Vec<vk::Extent3D>,
    層数: u32,
    元: vk::Buffer,
) -> graph::パス宣言<'static> {
    転送パスを作る(パス名, 画像, 段ごとの範囲, 層数, 元, 四成分テクセルのバイト数)
}

/// 層も縮小段も持たない2次元画像を書き込む。反射率積分表がこの形を取る。
pub(in crate::vulkan) fn 表の書き込みパスを作る(
    パス名: &'static str,
    画像: graph::画像ハンドル,
    範囲: vk::Extent3D,
    元: vk::Buffer,
    テクセルのバイト数: u64,
) -> graph::パス宣言<'static> {
    転送パスを作る(パス名, 画像, vec![範囲], 1, 元, テクセルのバイト数)
}

fn 転送パスを作る(
    パス名: &'static str,
    画像: graph::画像ハンドル,
    段ごとの範囲: Vec<vk::Extent3D>,
    層数: u32,
    元: vk::Buffer,
    テクセルのバイト数: u64,
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        パス名,
        Vec::new(),
        vec![(画像, graph::画像用途::転送先)],
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 画像ハンドル = 文脈.宣言済みの画像を参照する(画像);
            let 領域 = 領域を並べる(&段ごとの範囲, 層数, テクセルのバイト数);
            // 安全性: command_bufferは記録中、画像はTRANSFER_DST_OPTIMALへ遷移済み(用途宣言からグラフが導く)、
            // 元バッファは全段の全層ぶんのテクセル数の容量で確保し中身を書き終えている。
            unsafe {
                文脈.積み先().論理デバイス().cmd_copy_buffer_to_image(
                    文脈.積み先().コマンドバッファ(),
                    元,
                    画像ハンドル,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &領域,
                );
            }
        },
    )
}

/// 段ごとに1つの領域を作り、元バッファの位置を段の先頭から順に進める。
fn 領域を並べる(段ごとの範囲: &[vk::Extent3D], 層数: u32, テクセルのバイト数: u64) -> Vec<vk::BufferImageCopy> {
    let mut 位置 = 0u64;
    let mut 領域一覧 = Vec::with_capacity(段ごとの範囲.len());
    for (段番号, 範囲) in 段ごとの範囲.iter().enumerate() {
        let 段 = u32::try_from(段番号).unwrap_or_else(|_| panic!("縮小段の番号{段番号}がu32に収まらない"));
        領域一覧.push(
            vk::BufferImageCopy::default()
                .buffer_offset(位置)
                .image_subresource(層の部分範囲(段, 層数))
                .image_extent(*範囲),
        );
        位置 += u64::from(範囲.width) * u64::from(範囲.height) * u64::from(層数) * テクセルのバイト数;
    }
    領域一覧
}
