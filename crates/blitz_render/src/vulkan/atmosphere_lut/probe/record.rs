//! 読み戻し検査のコマンド記録。担当する工程は「本番と同じ2本の生成パスと、その後ろへ画像からバッファへのコピーを積み、
//! 1回の送信で実行して結果を読む」ことである。
//!
//! コピーを転送パスとしてグラフへ積むのは、生成の完了とコピーの間のバリアを本番と同じ導出に任せるためである。
//! 手で書いたバリアで済ませると、検査だけが本番と違う同期の下で通ってしまう。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::pass;
use crate::vulkan::atmosphere_lut::readback_buffer::LUT読み戻しバッファ;
use crate::vulkan::atmosphere_lut::大気LUT一式;
use crate::vulkan::graph;
use crate::vulkan::headless::ヘッドレスGPU環境;
use crate::vulkan::sync::フレームスロット添字;

use super::大気LUT読み戻し;

pub(super) fn 焼いて読み戻す(
    環境: &ヘッドレスGPU環境,
    一式: &大気LUT一式,
    透過率受け: &LUT読み戻しバッファ,
    多重散乱受け: &LUT読み戻しバッファ,
) -> Result<大気LUT読み戻し, レンダラーエラー> {
    let device = 環境.device();
    let 入力 = 一式.描画入力を作る(フレームスロット添字::先頭());
    環境.一度きりで実行する(|device, command_buffer| {
        let mut グラフ = graph::グラフ::新規();
        let 透過率 = 登録する(&mut グラフ, 入力.透過率画像, 入力.透過率ビュー, 入力.透過率寸法);
        let 多重散乱 = 登録する(&mut グラフ, 入力.多重散乱画像, 入力.多重散乱ビュー, 入力.多重散乱寸法);
        グラフ.パスを積む(pass::作る("透過率生成", 透過率, Vec::new(), &入力.透過率));
        let 透過率を読む = vec![(透過率, graph::画像用途::コンピュート読み)];
        グラフ.パスを積む(pass::作る("多重散乱生成", 多重散乱, 透過率を読む, &入力.多重散乱));
        グラフ.パスを積む(コピーパスを作る("透過率読み戻し", 透過率, 入力.透過率寸法, 透過率受け.handle));
        グラフ.パスを積む(コピーパスを作る(
            "多重散乱読み戻し",
            多重散乱,
            入力.多重散乱寸法,
            多重散乱受け.handle,
        ));
        graph::実行する(device, command_buffer, グラフ, None);
    })?;
    Ok(大気LUT読み戻し {
        透過率: 透過率受け.読み取る(device)?,
        多重散乱: 多重散乱受け.読み取る(device)?,
    })
}

fn 登録する(
    グラフ: &mut graph::グラフ<'_>, 画像: vk::Image, ビュー: vk::ImageView, 寸法: vk::Extent2D
) -> graph::画像ハンドル {
    グラフ.画像を登録する(画像, ビュー, graph::画像アスペクト::カラー, graph::前フレーム大気lut書き直後状態(), 寸法)
}

fn コピーパスを作る(
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
