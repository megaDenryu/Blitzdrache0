//! 光のにじみ縮小チェーンのパス宣言(判断41)。前処理(初段)はHDRを読みKaris平均+ソフト閾値で
//! 1/2解像度へ、2段目以降は13タップ縮小で1/2ずつ小さくする。

use ash::vk;

use super::fullscreen_draw;
use crate::vulkan::frame::光のにじみ描画入力;
use crate::vulkan::graph::{カラー添付列, パス宣言, パス種別, 画像ハンドル, 画像用途};

/// 前提: 添字は段番号。長さはピラミッドの段数上限(bloom_targets::段数上限)と一致させる。
const 縮小パス名一覧: [&str; 5] = [
    "光のにじみ前処理",
    "光のにじみ縮小2",
    "光のにじみ縮小3",
    "光のにじみ縮小4",
    "光のにじみ縮小5",
];

pub(super) fn 前処理を作る<'a>(
    hdr: 画像ハンドル, 縮小0: 画像ハンドル, 入力: &'a 光のにじみ描画入力, 寸法: vk::Extent2D
) -> パス宣言<'a> {
    作る(
        縮小パス名一覧[0],
        hdr,
        縮小0,
        入力.前処理pipeline,
        入力.前処理layout,
        入力.前処理set,
        寸法,
    )
}

/// `段`は書き先の段番号(1以上)。読み元は1段大きい縮小結果。
pub(super) fn 縮小を作る<'a>(
    段: usize,
    読み: 画像ハンドル,
    書き: 画像ハンドル,
    入力: &'a 光のにじみ描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    作る(
        縮小パス名一覧[段],
        読み,
        書き,
        入力.縮小pipeline,
        入力.縮小layout,
        入力.縮小set一覧[段 - 1],
        寸法,
    )
}

#[allow(clippy::too_many_arguments)]
fn 作る<'a>(
    パス名: &'static str,
    読み: 画像ハンドル,
    書き: 画像ハンドル,
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    set: vk::DescriptorSet,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        パス名,
        vec![(読み, 画像用途::シェーダー読み画素段)],
        vec![(書き, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: カラー添付列::色だけ(書き),
            深度: None,
            クリア指定: fullscreen_draw::黒クリア(),
        },
        move |文脈| {
            fullscreen_draw::コマンドを積む(文脈.device(), 文脈.コマンドバッファ(), pipeline, layout, set, 寸法, None);
        },
    )
}
