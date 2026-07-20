//! ブルーム輝度抽出パスの宣言(判断39): HDR画像をフラグメントで読み、
//! 閾値1.0超の成分を1/2解像度画像aへ書く。

use ash::vk;

use super::fullscreen_draw;
use crate::vulkan::frame::ブルーム描画入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    hdr: 画像ハンドル,
    ブルームa: 画像ハンドル,
    入力: &'a ブルーム描画入力,
    半解像度: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "ブルーム抽出",
        vec![(hdr, 画像用途::シェーダー読みフラグメント)],
        vec![(ブルームa, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(ブルームa),
            深度: None,
            クリア指定: fullscreen_draw::黒クリア(),
        },
        move |文脈| {
            fullscreen_draw::コマンドを積む(
                文脈.device(),
                文脈.コマンドバッファ(),
                入力.抽出pipeline,
                入力.抽出layout,
                入力.抽出set,
                半解像度,
                None,
            );
        },
    )
}
