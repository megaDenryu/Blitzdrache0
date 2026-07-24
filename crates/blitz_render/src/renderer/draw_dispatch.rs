//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`を決め、
//! `draw_inputs`で組み立てた入力束とフレーム画像一式を`vulkan::frame::描画する`へ渡す。
//! ビュー射影行列等はdraw_execute.rsが事前にUBOへ書き込み済み(判断24)のため、
//! ここではフレーム添字に対応するディスクリプタセットを選ぶだけでよい。

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム段階;
use crate::vulkan;
use crate::vulkan::frame::{UI描画入力, 任意描画入力, 描画対象入力, 描画方式};

impl レンダラー {
    /// 戻り値: 提示劣化の有無と、このフレームで書いたGPUタイムスタンプの
    /// 「パス名→クエリ開始添字」対応(判断30。計測無効なら空配列)。
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(super) fn 現在の画像で描画する(
        &mut self,
        添字: u32,
        フレーム添字: usize,
        クリア色: クリアカラー,
        露出: f32,
        布介入件数: u32,
        読み戻し要求: bool,
        ui入力: Option<&UI描画入力>,
    ) -> Result<(bool, Vec<(&'static str, u32)>), レンダラーエラー> {
        let 添字usize = usize::try_from(添字).unwrap_or_else(|_| panic!("スワップチェーン画像添字がusizeに収まらない: {添字}"));

        let 描画方式 = self.描画方式を決める(読み戻し要求)?;
        let ポスト入力 = self.ポスト入力束を組み立てる(フレーム添字, 露出);
        let 布入力 = self.布入力を組み立てる(フレーム添字, 布介入件数);
        let 粒子入力 = self.粒子描画入力を組み立てる(フレーム添字);
        let クエリプール = self.gpu計測.as_ref().map(|計測| 計測.クエリプール(フレーム添字));
        let 画像一式 = self.フレーム画像一式を組み立てる(添字usize);
        let 提示id = self.実表示計測.提示idを発番する();
        self.描画入力作業領域を更新する(フレーム添字);

        vulkan::frame::描画する(
            &self.device,
            self.queue,
            self.command_buffer一覧[フレーム添字],
            &self.フレーム構成,
            self.提示先を組み立てる(添字, 提示id),
            &画像一式,
            self.swapchain.寸法,
            クリア色,
            self.pipeline.handle,
            描画対象入力 {
                ジオメトリ: &self.ジオメトリ入力作業領域,
                シャドウ: &self.シャドウ入力作業領域,
            },
            任意描画入力 {
                スキニング: ポスト入力.スキニング.as_ref(),
                布: 布入力.as_ref(),
                粒子: 粒子入力.as_ref(),
                ブルーム: ポスト入力.ブルーム.as_ref(),
                トーンマップ: ポスト入力.トーンマップ.as_ref(),
                ui: ui入力,
            },
            描画方式,
            クエリプール,
            self.同期入力を組み立てる(フレーム添字, 添字usize),
        )
    }

    fn 描画方式を決める(&self, 読み戻し要求: bool) -> Result<描画方式, レンダラーエラー> {
        if 読み戻し要求 {
            if !self.フレーム構成.含む(フレーム段階::読み戻し) {
                return Err(レンダラーエラー::読み戻し段階なし);
            }
            let バッファ = self
                .読み戻しバッファ
                .as_ref()
                .unwrap_or_else(|| panic!("読み戻し要求時に読み戻しバッファが未確保だった"));
            Ok(描画方式::読み戻し {
                バッファ: バッファ.handle
            })
        } else {
            Ok(描画方式::通常)
        }
    }
}
