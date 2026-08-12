//! 合成入力4枚を時間再構成のパスが読む画像へ書き戻す転送パスの宣言。受け取るのは4つのハンドルと注入の入力、返すのはパス宣言1本である。
//!
//! 注意: 積む位置は時間再構成のパスの直前である。前へ置くとシーン描画と空のパスが今のフレームの色と動きベクトルを上書きし、後ろへ置くと再構成が本番の描画結果を読む。どちらも「与えた入力とは違うものから作った結果」を正本と突き合わせることになる。積む位置を持つのは`graph_build`の側であり、この宣言は順序を1つも知らない。
//!
//! 4枚を1本のパスにまとめるのは、4枚が同時に揃っていなければ突き合わせが成立しないためである。パスを分けると、レンダーグラフの並びの途中に「3枚だけ差し替わった状態」が現れる。

use ash::vk;

use crate::vulkan::graph::{
    GPU命令の積み先と宣言済み資源の取り出し口, パス宣言, パス種別, 画像ハンドル, 画像用途
};

/// 転送パスが要る4本のハンドルと寸法。
#[derive(Clone, Copy)]
pub(crate) struct 合成入力の注入入力 {
    pub(crate) 今のフレームの色: vk::Buffer,
    pub(crate) 履歴: vk::Buffer,
    pub(crate) 動きベクトル: vk::Buffer,
    pub(crate) 深度: vk::Buffer,
    pub(crate) 寸法: vk::Extent2D,
}

/// 書き戻す先の4つのハンドル。位置引数で並べると対応を目で追う必要が出るため、名前付きで渡す。
#[derive(Clone, Copy)]
pub(crate) struct 合成入力の書き戻し先 {
    pub(crate) 今のフレームの色: 画像ハンドル,
    pub(crate) 履歴読み: 画像ハンドル,
    pub(crate) 動きベクトル: 画像ハンドル,
    pub(crate) 深度: 画像ハンドル,
}

pub(crate) fn 合成入力の注入を作る<'a>(
    書き戻し先: 合成入力の書き戻し先, 入力: 合成入力の注入入力
) -> パス宣言<'a> {
    パス宣言::生成する(
        "時間再構成の合成入力の注入",
        Vec::new(),
        vec![
            (書き戻し先.今のフレームの色, 画像用途::転送先),
            (書き戻し先.履歴読み, 画像用途::転送先),
            (書き戻し先.動きベクトル, 画像用途::転送先),
            (書き戻し先.深度, 画像用途::転送先),
        ],
        Vec::new(),
        Vec::new(),
        パス種別::転送,
        move |文脈| コピーを積む(文脈, 書き戻し先, 入力),
    )
}

fn コピーを積む(
    文脈: &GPU命令の積み先と宣言済み資源の取り出し口, 書き戻し先: 合成入力の書き戻し先, 入力: 合成入力の注入入力
) {
    let 色の面 = vk::ImageAspectFlags::COLOR;
    let 組一覧 = [
        (書き戻し先.今のフレームの色, 入力.今のフレームの色, 色の面),
        (書き戻し先.履歴読み, 入力.履歴, 色の面),
        (書き戻し先.動きベクトル, 入力.動きベクトル, 色の面),
        (書き戻し先.深度, 入力.深度, vk::ImageAspectFlags::DEPTH),
    ];
    for (ハンドル, バッファ, 面) in 組一覧 {
        一枚を写す(文脈, ハンドル, バッファ, 面, 入力.寸法);
    }
}

fn 一枚を写す(
    文脈: &GPU命令の積み先と宣言済み資源の取り出し口,
    ハンドル: 画像ハンドル,
    バッファ: vk::Buffer,
    面: vk::ImageAspectFlags,
    寸法: vk::Extent2D,
) {
    let 画像 = 文脈.画像を解決する(ハンドル);
    let 領域 = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(面)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: 寸法.width,
            height: 寸法.height,
            depth: 1,
        });
    let 領域一覧 = [領域];
    // 安全性: command_bufferは記録中、対象の画像はグラフの導いたバリアでTRANSFER_DST_OPTIMALへ遷移済み、
    // バッファは同じ寸法の成分列で確保済みである(`合成入力の注入一式::生成する`が同じ寸法から作る)。
    unsafe {
        文脈.積み先().論理デバイス().cmd_copy_buffer_to_image(
            文脈.積み先().コマンドバッファ(),
            バッファ,
            画像,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &領域一覧,
        );
    }
}
