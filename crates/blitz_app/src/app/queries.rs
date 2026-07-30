//! アプリ状態の読み出しと終了処理(main.rsの終了処理する等が使う照会メソッド群)。
//! どの報告が求められたかの問い合わせは`report_request`が持つ。

mod report_request;

use blitz_render::{
    CPU区間時間, GPUメモリ統計, レンダラー, 大気のベイク済み画像生成パス数の記録, 実表示観測, 実表示計測状況, 描画発行内訳, 検証カウンタ,
};

use super::アプリ;
use crate::error::起動エラー;

impl アプリ {
    /// resumed/window_event内で発生した起動時エラーを取り出す。
    pub(crate) fn 起動時エラーを取り出す(&mut self) -> Option<起動エラー> {
        self.起動時エラー.take()
    }

    /// 破棄後に読むための検証カウンタ。レンダラー未生成なら`None`(一度も描画していないためvalidationメッセージも発生し得ない)。
    pub(crate) fn 検証カウンタを取得する(&self) -> Option<検証カウンタ> {
        self.レンダラー.as_ref().map(レンダラー::検証カウンタを取得する)
    }

    pub(crate) fn gpuメモリ統計を取得する(&self) -> Option<GPUメモリ統計> {
        self.レンダラー.as_ref().map(レンダラー::gpuメモリ統計を取得する)
    }

    /// 最終フレームの描画発行内訳。レンダラー破棄前に呼ぶこと。
    pub(crate) fn 描画発行内訳を取得する(&self) -> Option<描画発行内訳> {
        self.レンダラー.as_ref().map(レンダラー::描画発行内訳を取得する)
    }

    /// 大気のベイク済み画像生成パスの実行数の記録。レンダラー未生成なら1フレームも数えていないため`None`。
    pub(crate) fn 大気のベイク済み画像生成パス数の記録を取得する(
        &self,
    ) -> Option<&大気のベイク済み画像生成パス数の記録> {
        self.レンダラー.as_ref().map(レンダラー::大気のベイク済み画像生成パス数の記録を取得する)
    }

    /// 起動から現在までに個体の段が入れ替わった延べ回数。段の記憶を持つ可視判定の配線から集める。
    pub(crate) fn 個体詳細段切替回数を取得する(&self) -> u64 {
        self.可視判定.段切替回数()
    }

    /// 可視判定と段選択へ登録されている群の件数。束の解除で材料と段の記憶が漏れなく消えたことをこの数で見る。
    pub(crate) fn 可視材料登録数を取得する(&self) -> usize {
        self.可視判定.登録数()
    }

    /// 最終フレームに可視個体の選別が数えた計数。レンダラーの描画発行内訳と出どころが違う2つ目の計器である。
    pub(crate) fn 可視個体の選別の計数を取得する(&self) -> blitz_engine::可視判定計数 {
        self.可視判定.直近の計数()
    }

    /// ディスクから実行時シーンを読んだ回数と、そのうち1フレーム目以降に起きた回数。
    pub(crate) fn シーン読込回数を取得する(&self) -> (u64, u64) {
        (self.シーン読込計数.総数(), self.シーン読込計数.起動後())
    }

    /// パス別の移動平均GPU時間(ミリ秒)。レンダラー破棄前に呼ぶこと(判断30)。
    pub(crate) fn パス別gpu時間を取得する(&self) -> Vec<(&'static str, f64)> {
        self.レンダラー.as_ref().map(レンダラー::パス別gpu時間を取得する).unwrap_or_default()
    }

    /// `--report-frame-times`で収集した、ウォームアップ後のCPU側フレーム間隔分布を返す。
    pub(crate) fn フレーム時間統計を取得する(&self) -> Option<super::frame_timing::フレーム時間統計> {
        self.フレーム間隔計測.as_ref().and_then(super::frame_timing::フレーム間隔計測::集計する)
    }

    /// `--report-instance-sections`で収集した、可視個体の選別の走査がメインスレッドを占めた時間の分布。
    pub(crate) fn 可視個体の選別の区間統計を取得する(&self) -> Option<super::frame_timing::フレーム時間統計> {
        self.可視個体の選別の計測.as_ref().and_then(super::section_timing::区間計測::集計する)
    }

    pub(crate) fn レンダラーcpu区間時間を取得する(&self) -> &[CPU区間時間] {
        self.レンダラー.as_ref().map_or(&[], レンダラー::cpu区間時間一覧を取得する)
    }

    /// 実表示時刻計測の対応状況。レンダラー未生成なら拡張の有無を確かめられていないため`None`。
    pub(crate) fn 実表示計測状況を取得する(&self) -> Option<実表示計測状況> {
        self.レンダラー.as_ref().map(レンダラー::実表示計測状況を取得する)
    }

    pub(crate) fn 実表示観測一覧を取得する(&self) -> &[実表示観測] {
        self.レンダラー.as_ref().map_or(&[], レンダラー::実表示観測一覧を取得する)
    }

    /// 固定経路実行で観測した転送量・処理時間・最大使用量。`--streaming`が無ければ調停自体が無く`None`。
    pub(crate) fn ストリーミング要約を取得する(&self) -> Option<super::ストリーミング要約> {
        let 見送りフレーム数 = self.レンダラー.as_ref().map_or(0, レンダラー::見送りフレーム数を取得する);
        self.ストリーミング.as_ref().map(|配線| 配線.要約を作る(見送りフレーム数))
    }

    /// イベントループ終了後に呼び、破棄順序を明示する。
    pub(crate) fn レンダラーを破棄する(&mut self) {
        self.レンダラー = None;
    }
}
