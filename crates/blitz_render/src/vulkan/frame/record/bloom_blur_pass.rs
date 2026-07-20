//! ブルーム分離ガウシアンぼかしパスの宣言(判断39)。横(a→b)と縦(b→a)で同じ
//! パイプラインを使い、方向はプッシュ定数(float2、テクセル単位)で切り替える。

use ash::vk;

use super::fullscreen_draw;
use crate::vulkan::frame::ブルーム描画入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) struct ぼかし指定 {
    pub(super) パス名: &'static str,
    pub(super) 方向: [f32; 2],
    pub(super) セットは横か: bool,
}

pub(super) const 横ぼかし: ぼかし指定 = ぼかし指定 { パス名: "ブルーム横ぼかし", 方向: [1.0, 0.0], セットは横か: true };
pub(super) const 縦ぼかし: ぼかし指定 = ぼかし指定 { パス名: "ブルーム縦ぼかし", 方向: [0.0, 1.0], セットは横か: false };

pub(super) fn 作る<'a>(
    読み: 画像ハンドル,
    書き: 画像ハンドル,
    入力: &'a ブルーム描画入力,
    指定: ぼかし指定,
    半解像度: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        指定.パス名,
        vec![(読み, 画像用途::シェーダー読みフラグメント)],
        vec![(書き, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス {
            カラー: Some(書き),
            深度: None,
            クリア指定: fullscreen_draw::黒クリア(),
        },
        move |文脈| {
            let mut 方向バイト列 = [0u8; 8];
            方向バイト列[..4].copy_from_slice(&指定.方向[0].to_le_bytes());
            方向バイト列[4..].copy_from_slice(&指定.方向[1].to_le_bytes());
            let セット = if 指定.セットは横か { 入力.横set } else { 入力.縦set };
            fullscreen_draw::コマンドを積む(
                文脈.device(),
                文脈.コマンドバッファ(),
                入力.ぼかしpipeline,
                入力.ぼかしlayout,
                セット,
                半解像度,
                Some(&方向バイト列),
            );
        },
    )
}
