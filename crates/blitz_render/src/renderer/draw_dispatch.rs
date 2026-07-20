//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`を決め、
//! `draw_inputs`で組み立てた入力束とフレーム画像一式を`vulkan::frame::描画する`へ渡す。
//! ビュー射影行列等はdraw_execute.rsが事前にUBOへ書き込み済み(判断24)のため、
//! ここではフレーム添字に対応するディスクリプタセットを選ぶだけでよい。

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::frame::{トーンマップ描画入力, ブルーム画像, ブルーム描画入力, フレーム画像一式, 描画方式, UI描画入力};

impl レンダラー {
    /// 戻り値: 提示劣化の有無と、このフレームで書いたGPUタイムスタンプの
    /// 「パス名→クエリ開始添字」対応(判断30。計測無効なら空配列)。
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(super) fn 現在の画像で描画する(
        &self,
        添字: u32,
        フレーム添字: usize,
        クリア色: クリアカラー,
        露出: f32,
        読み戻し要求: bool,
        ui入力: Option<&UI描画入力>,
    ) -> Result<(bool, Vec<(&'static str, u32)>), レンダラーエラー> {
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

        let 入力束 = self.描画入力束を組み立てる(フレーム添字);
        let クエリプール = self.gpu計測.as_ref().map(|計測| 計測.クエリプール(フレーム添字));

        // ポストプロセスの入力と画像は対で生成される(判断38。graph_build側の不変条件)。
        let トーンマップ入力 = self.トーンマップ.as_ref().map(|一式| トーンマップ描画入力 {
            pipeline: 一式.pipeline,
            layout: 一式.layout,
            ディスクリプタセット: 一式.descriptor_set,
            露出,
        });
        let ブルーム入力 = self.ブルーム.as_ref().map(|一式| ブルーム描画入力 {
            抽出pipeline: 一式.抽出pipeline,
            抽出layout: 一式.抽出layout,
            ぼかしpipeline: 一式.ぼかしpipeline,
            ぼかしlayout: 一式.ぼかしlayout,
            抽出set: 一式.抽出set,
            横set: 一式.横set,
            縦set: 一式.縦set,
        });
        let 画像一式 = フレーム画像一式 {
            スワップチェーン画像: self.swapchain.画像一覧[添字usize],
            スワップチェーンビュー: self.swapchain.画像ビュー一覧[添字usize],
            深度画像: self.深度バッファ.画像,
            深度ビュー: self.深度バッファ.画像ビュー,
            シャドウマップ画像: self.シャドウマップ.画像,
            シャドウマップビュー: self.シャドウマップ.画像ビュー,
            hdr: self.hdrターゲット.as_ref().map(|hdr| (hdr.画像, hdr.画像ビュー)),
            ブルーム: self.ブルームターゲット.as_ref().map(|一式| ブルーム画像 {
                a画像: 一式.a.画像,
                aビュー: 一式.a.画像ビュー,
                b画像: 一式.b.画像,
                bビュー: 一式.b.画像ビュー,
                寸法: 一式.寸法,
            }),
        };

        vulkan::frame::描画する(
            &self.device,
            self.queue,
            self.command_buffer一覧[フレーム添字],
            &self.swapchain_loader,
            self.swapchain.handle,
            添字,
            &画像一式,
            self.swapchain.寸法,
            クリア色,
            self.pipeline.handle,
            &入力束.ジオメトリ,
            &入力束.シャドウ,
            入力束.粒子.as_ref(),
            ブルーム入力.as_ref(),
            トーンマップ入力.as_ref(),
            ui入力,
            描画方式,
            クエリプール,
            self.フレーム同期.取得セマフォ(フレーム添字),
            self.提示同期.提示セマフォ(添字usize),
            self.フレーム同期.フェンス(フレーム添字),
        )
    }
}
