//! 光のにじみの各ディスクリプタセットへ読み元ビューを書く。
//! 前処理set←HDR、縮小set[i]←縮小[i](縮小[i+1]への縮小パスの読み元)、
//! 拡大set[i]←(1段小さい拡大結果または縮小最終段, 縮小[i])。

use ash::vk;

use super::光のにじみ一式;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::descriptor::{宣言から割り当てたセット, 結ぶ現物};

impl 光のにじみ一式 {
    /// 前提: 呼び出し時点でGPUがこれらのディスクリプタセットを使用していないこと
    /// (`ディスクリプタを作り直す`経由でのみ呼ばれ、その前提を引き継ぐ)。
    pub(super) fn ビューを書く(&self, device: &ash::Device, hdrビュー: vk::ImageView, ピラミッド: &光のにじみピラミッド) {
        let セット群 = self.確保済みのセット群();
        self.読み元1枚を書く(device, &セット群.前処理set, hdrビュー);
        for (添字, set) in セット群.縮小set一覧.iter().enumerate() {
            self.読み元1枚を書く(device, set, ピラミッド.縮小一覧[添字].画像ビュー);
        }
        for (添字, set) in セット群.拡大set一覧.iter().enumerate() {
            let 小さい方 = if 添字 + 1 < セット群.拡大set一覧.len() {
                ピラミッド.拡大一覧[添字 + 1].画像ビュー
            } else {
                ピラミッド.縮小一覧[添字 + 1].画像ビュー
            };
            set.書き込み先(device)
                .並びの位置ごとに結ぶ([self.読み元(小さい方), self.読み元(ピラミッド.縮小一覧[添字].画像ビュー)]);
        }
    }

    fn 読み元1枚を書く(&self, device: &ash::Device, set: &宣言から割り当てたセット<1>, ビュー: vk::ImageView) {
        set.書き込み先(device).並びの位置ごとに結ぶ([self.読み元(ビュー)]);
    }

    fn 読み元(&self, ビュー: vk::ImageView) -> 結ぶ現物 {
        結ぶ現物::サンプラー付きの画像 {
            ビュー,
            サンプラー: self.sampler,
            レイアウト: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        }
    }
}
