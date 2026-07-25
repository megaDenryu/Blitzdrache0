//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`を決め、各サブシステムから集めた入力束と
//! フレーム画像一式を`vulkan::frame::描画する`へ渡す。
//! ビュー射影行列等はdraw_execute.rsが事前にUBOへ書き込み済み(判断24)のため、
//! ここではフレーム添字に対応するディスクリプタセットを選ぶだけでよい。

use super::frame_progress::フレームスロット資源;
use super::presentation::取得済み提示;
use super::scene_draw_resources::作業領域更新入力;
use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム段階;
use crate::vulkan;
use crate::vulkan::frame::{UI描画入力, 任意描画入力, 描画方式};
use crate::vulkan::skinning::スキニング一式;
use crate::vulkan::sync::フレームスロット添字;

impl レンダラー {
    /// 戻り値: 提示劣化の有無と、このフレームで書いたGPUタイムスタンプの
    /// 「パス名→クエリ開始添字」対応(判断30。計測無効なら空配列)。
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(super) fn 現在の画像で描画する(
        &mut self,
        取得済み: &取得済み提示,
        スロット資源: &フレームスロット資源,
        クリア色: クリアカラー,
        露出: f32,
        布介入件数: u32,
        読み戻し要求: bool,
        ui入力: Option<&UI描画入力>,
    ) -> Result<(bool, Vec<(&'static str, u32)>), レンダラーエラー> {
        let フレーム添字 = スロット資源.スロット;

        let 描画方式 = self.描画方式を決める(読み戻し要求)?;
        let ポスト入力 = self.ポスト処理.as_ref().map(|一式| 一式.描画入力を作る(露出));
        let スキニング入力 = self.スキニング.as_ref().map(|一式| 一式.描画入力を作る(フレーム添字));
        let 布入力 = self.布入力を組み立てる(フレーム添字, 布介入件数);
        let 粒子入力 = self.粒子.as_ref().map(|一式| 一式.描画入力を作る(フレーム添字));
        let クエリプール = self.gpu計測.as_ref().map(|計測| 計測.クエリプール(フレーム添字));
        let 画像一式 = self.フレーム画像一式を組み立てる(取得済み);
        let 提示id = self.実表示計測.提示idを発番する();
        let 作業領域入力 = self.作業領域更新入力を作る(フレーム添字);
        self.シーン描画資源.作業領域を更新する(&作業領域入力);

        vulkan::frame::描画する(
            self.環境.device(),
            self.環境.queue(),
            スロット資源.command_buffer,
            &self.フレーム構成,
            self.提示先を組み立てる(取得済み, 提示id),
            &画像一式,
            self.提示.寸法(),
            クリア色,
            self.pipeline.handle,
            self.シーン描画資源.描画対象入力を作る(),
            任意描画入力 {
                スキニング: スキニング入力.as_ref(),
                布: 布入力.as_ref(),
                粒子: 粒子入力.as_ref(),
                ブルーム: ポスト入力.as_ref().map(|入力| &入力.ブルーム),
                トーンマップ: ポスト入力.as_ref().map(|入力| &入力.トーンマップ),
                ui: ui入力,
            },
            描画方式,
            クエリプール,
            self.同期入力を組み立てる(スロット資源, 取得済み),
        )
    }

    /// 描画入力作業領域の充填に要る、描画対象資源の外の値を集める。スキン付きシーンでは先頭対象の頂点入力をスキン済みバッファへ差し替える(判断44)。
    fn 作業領域更新入力を作る(&self, フレーム添字: フレームスロット添字) -> 作業領域更新入力 {
        作業領域更新入力 {
            フレーム添字,
            スキン済み頂点バッファ: self.スキニング.as_ref().map(スキニング一式::出力バッファ),
            シーンlayout: self.pipeline.layout,
            シャドウpipeline: self.シャドウパイプライン.handle,
            シャドウlayout: self.シャドウパイプライン.layout,
        }
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
