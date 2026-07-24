//! アプリ状態の読み出しと終了処理(main.rsの終了処理する等が使う照会メソッド群)。

use blitz_render::{CPU区間時間, GPUメモリ統計, レンダラー, 実表示観測, 実表示計測状況, 検証カウンタ};

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

    /// `--report-gpu-times`が指定されたか。
    pub(crate) fn gpu時間報告が必要か(&self) -> bool {
        self.gpu時間報告
    }

    pub(crate) fn フレーム時間報告が必要か(&self) -> bool {
        self.フレーム間隔計測.is_some()
    }

    pub(crate) fn gpuメモリ報告が必要か(&self) -> bool {
        self.gpuメモリ報告
    }

    pub(crate) fn gpuメモリ統計を取得する(&self) -> Option<GPUメモリ統計> {
        self.レンダラー.as_ref().map(レンダラー::gpuメモリ統計を取得する)
    }

    /// パス別の移動平均GPU時間(ミリ秒)。レンダラー破棄前に呼ぶこと(判断30)。
    pub(crate) fn パス別gpu時間を取得する(&self) -> Vec<(&'static str, f64)> {
        self.レンダラー.as_ref().map(レンダラー::パス別gpu時間を取得する).unwrap_or_default()
    }

    /// `--report-frame-times`で収集した、ウォームアップ後のCPU側フレーム間隔分布を返す。
    pub(crate) fn フレーム時間統計を取得する(&self) -> Option<super::frame_timing::フレーム時間統計> {
        self.フレーム間隔計測.as_ref().and_then(super::frame_timing::フレーム間隔計測::集計する)
    }

    pub(crate) fn レンダラーcpu区間時間を取得する(&self) -> &[CPU区間時間] {
        self.レンダラー.as_ref().map_or(&[], レンダラー::cpu区間時間一覧を取得する)
    }

    /// `--report-display-timing`が指定されたか。
    pub(crate) fn 実表示時間報告が必要か(&self) -> bool {
        self.実表示時間報告
    }

    /// 実表示時刻計測の対応状況。レンダラー未生成なら拡張の有無を確かめられていないため`None`。
    pub(crate) fn 実表示計測状況を取得する(&self) -> Option<実表示計測状況> {
        self.レンダラー.as_ref().map(レンダラー::実表示計測状況を取得する)
    }

    pub(crate) fn 実表示観測一覧を取得する(&self) -> &[実表示観測] {
        self.レンダラー.as_ref().map_or(&[], レンダラー::実表示観測一覧を取得する)
    }

    /// イベントループ終了後に呼び、破棄順序を明示する。
    pub(crate) fn レンダラーを破棄する(&mut self) {
        self.レンダラー = None;
    }
}
