//! 明るさの圧縮パスの宣言(判断38・39): HDR画像と光のにじみ結果(a)を画素段で読み、
//! 全画面三角形でスワップチェーンへ露出+ACES+光のにじみ加算の結果を書く。
//! 記録クロージャの中身は`fullscreen_draw`が担う(begin/end renderingは実行器が担う)。

use ash::vk;

use super::fullscreen_draw;
use crate::vulkan::frame::明るさの圧縮描画入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    hdr: 画像ハンドル,
    光のにじみa: 画像ハンドル,
    スワップチェーン: 画像ハンドル,
    入力: &'a 明るさの圧縮描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "明るさの圧縮",
        vec![(hdr, 画像用途::シェーダー読み画素段), (光のにじみa, 画像用途::シェーダー読み画素段)],
        vec![(スワップチェーン, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(スワップチェーン),
            深度: None,
            クリア指定: fullscreen_draw::黒クリア(),
        },
        move |文脈| {
            let 露出バイト列 = 入力.露出.to_le_bytes();
            fullscreen_draw::コマンドを積む(
                文脈.device(),
                文脈.コマンドバッファ(),
                入力.pipeline,
                入力.layout,
                入力.ディスクリプタセット,
                寸法,
                Some(&露出バイト列),
            );
        },
    )
}
