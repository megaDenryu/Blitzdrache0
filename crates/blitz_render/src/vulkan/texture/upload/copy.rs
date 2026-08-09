//! bufferからimageへのコピー領域の組み立てと発行。担当する工程は「ステージングバッファの中身をどの段のどの範囲へ写すかを
//! 決めて1回のコピーコマンドとして積む」ことであり、受け取るのは積み先のコマンドバッファと画像と素材、返すものは無い。
//!
//! 注意: 圧縮形式のコピー領域は、Vulkanが定める2つの規則を守る必要がある。1つは、コピーする範囲の寸法がブロックの一辺の
//! 倍数であるか、その段の実寸に届いていることである。もう1つは、バッファ内の開始位置が1ブロックのバイト数の倍数であることである。
//! ここでは段の実寸をそのまま範囲に渡し、開始位置を段のバイト数の積み上げで求めるため、どちらも満たす。
//! BC1の1段のバイト数は1ブロックの8バイトの倍数であり、その和も8の倍数であるためである。
//! 4の倍数でない幅の段でも右端のブロックが余白を含んだ8バイトを占め、その並びは素材の生成が段のバイト数として検査済みである。
//! この規則の綴りは仕様書の本文からの引用ではない。validation layerが同じ規則を検査するため、成立は検収の実行が反証する。

use ash::vk;

use crate::texture_material::level_extent::縮小段の幅と高さを求める;
use crate::texture_material::テクスチャ素材;

pub(super) fn 原寸の段を画像へコピーする(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    幅: u32,
    高さ: u32,
) {
    let 領域 = コピー領域を組み立てる(0, 0, 幅, 高さ);
    // 安全性: command_bufferは積み込み中。imageはTRANSFER_DST_OPTIMALへ遷移済み。
    // ステージングバッファは呼び出し元が原寸の画素列と同じ長さで確保・書き込み済み。
    unsafe {
        device.cmd_copy_buffer_to_image(
            command_buffer,
            ステージングバッファ,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[領域],
        );
    }
}

/// 素材が運ぶ全段を、段0から順に並べたステージングバッファの中身から1回のコマンドで写す。
pub(super) fn 全段を画像へコピーする(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    素材: &テクスチャ素材,
) {
    let mut 開始位置: vk::DeviceSize = 0;
    let mut 領域一覧: Vec<vk::BufferImageCopy> = Vec::with_capacity(素材.段ごとのバイト列().len());
    for (添字, バイト列) in 素材.段ごとのバイト列().iter().enumerate() {
        let 段番号 = u32::try_from(添字).unwrap_or_else(|_| panic!("縮小段の段番号{添字}が32ビットに収まらない"));
        let (段の幅, 段の高さ) = 縮小段の幅と高さを求める(素材.幅(), 素材.高さ(), 段番号);
        領域一覧.push(コピー領域を組み立てる(開始位置, 段番号, 段の幅, 段の高さ));
        let 段のバイト数 = vk::DeviceSize::try_from(バイト列.len()).unwrap_or_else(|_| panic!("縮小段{段番号}のバイト数が64ビットに収まらない"));
        開始位置 += 段のバイト数;
    }
    // 安全性: command_bufferは積み込み中。imageは全レベルがTRANSFER_DST_OPTIMALへ遷移済み。
    // 領域一覧の開始位置と長さの総和は、呼び出し元が同じ段の列から確保・書き込んだステージングバッファの容量に一致する。
    unsafe {
        device.cmd_copy_buffer_to_image(
            command_buffer,
            ステージングバッファ,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &領域一覧,
        );
    }
}

/// バッファ内の並びに隙間を作らないため、行の長さと高さを0にして「範囲の寸法どおりに詰まっている」ことを表す。
fn コピー領域を組み立てる(開始位置: vk::DeviceSize, 段番号: u32, 段の幅: u32, 段の高さ: u32) -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .buffer_offset(開始位置)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(段番号)
                .base_array_layer(0)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: 段の幅,
            height: 段の高さ,
            depth: 1,
        })
}
