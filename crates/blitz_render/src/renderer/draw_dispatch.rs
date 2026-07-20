//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`と
//! 頂点/インデックス/ディスクリプタセットの`ジオメトリ入力`をここで組み立てる。
//! ビュー射影行列等はdraw.rsが事前にUBOへ書き込み済み(判断24)のため、
//! ここではフレーム添字に対応するディスクリプタセットを選ぶだけでよい。

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::frame::{ジオメトリ入力, 描画方式};

impl レンダラー {
    pub(super) fn 現在の画像で描画する(
        &self,
        添字: u32,
        フレーム添字: usize,
        クリア色: クリアカラー,
        読み戻し要求: bool,
    ) -> Result<bool, レンダラーエラー> {
        let 添字usize = usize::try_from(添字)
            .unwrap_or_else(|_| panic!("スワップチェーン画像添字がusizeに収まらない: {添字}"));

        let 描画方式 = if 読み戻し要求 {
            let バッファ = self
                .読み戻しバッファ
                .as_ref()
                .unwrap_or_else(|| panic!("読み戻し要求時に読み戻しバッファが未確保だった"));
            描画方式::読み戻し { バッファ: バッファ.handle }
        } else {
            描画方式::通常
        };

        let ジオメトリ入力 = ジオメトリ入力 {
            頂点バッファ: self.ジオメトリ.頂点バッファ,
            インデックスバッファ: self.ジオメトリ.インデックスバッファ,
            インデックス数: self.ジオメトリ.インデックス数,
            layout: self.pipeline.layout,
            ディスクリプタセット: self.ディスクリプタ.set(フレーム添字),
        };

        vulkan::frame::描画する(
            &self.device,
            self.queue,
            self.command_buffer一覧[フレーム添字],
            &self.swapchain_loader,
            self.swapchain.handle,
            添字,
            self.swapchain.画像一覧[添字usize],
            self.swapchain.画像ビュー一覧[添字usize],
            self.深度バッファ.画像,
            self.深度バッファ.画像ビュー,
            self.swapchain.寸法,
            クリア色,
            self.pipeline.handle,
            &ジオメトリ入力,
            描画方式,
            self.フレーム同期.取得セマフォ(フレーム添字),
            self.提示同期.提示セマフォ(添字usize),
            self.フレーム同期.フェンス(フレーム添字),
        )
    }
}
