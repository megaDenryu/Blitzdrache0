//! フレーム送信に渡す画像、提示先、同期プリミティブをレンダラー資源から組み立てる。

use super::レンダラー;
use crate::vulkan::frame::{フレーム画像一式, ブルーム画像, 同期入力, 提示先};

impl レンダラー {
    pub(super) fn 提示先を組み立てる(&self, 画像添字: u32, 提示id: Option<u64>) -> 提示先<'_> {
        提示先 {
            loader: &self.swapchain_loader,
            swapchain: self.swapchain.handle,
            画像添字,
            提示id,
        }
    }

    pub(super) fn 同期入力を組み立てる(&self, フレーム添字: usize, 画像添字: usize) -> 同期入力 {
        同期入力 {
            取得セマフォ: self.フレーム同期.取得セマフォ(フレーム添字),
            提示セマフォ: self.提示同期.提示セマフォ(画像添字),
            描画完了フェンス: self.フレーム同期.フェンス(フレーム添字),
        }
    }

    pub(super) fn フレーム画像一式を組み立てる(&self, 添字: usize) -> フレーム画像一式 {
        フレーム画像一式 {
            スワップチェーン画像: self.swapchain.画像一覧[添字],
            スワップチェーンビュー: self.swapchain.画像ビュー一覧[添字],
            深度画像: self.深度バッファ.画像,
            深度ビュー: self.深度バッファ.画像ビュー,
            シャドウマップ画像: self.シャドウマップ.画像,
            シャドウマップビュー: self.シャドウマップ.画像ビュー,
            hdr: self.hdrターゲット.as_ref().map(|hdr| (hdr.画像, hdr.画像ビュー)),
            ブルーム: self.ブルームピラミッド.as_ref().map(|ピラミッド| ブルーム画像 {
                縮小一覧: ピラミッド.縮小一覧.iter().map(|画像| (画像.画像, 画像.画像ビュー)).collect(),
                拡大一覧: ピラミッド.拡大一覧.iter().map(|画像| (画像.画像, 画像.画像ビュー)).collect(),
                寸法一覧: ピラミッド.寸法一覧.clone(),
            }),
        }
    }
}
