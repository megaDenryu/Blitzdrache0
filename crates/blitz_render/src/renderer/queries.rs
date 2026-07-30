//! レンダラーが外部へ公開する読み取り専用アクセサ。すべて`&self`でレンダラーの状態を変えないため、
//! 描画ループの途中でも終了後でも呼べる(検証カウンタだけは破棄後に読む規律がある)。
//!
//! 参照: 計測の有効化は`measurement_control.rs`、寸法変更の通知は`reconstruct.rs`にある。

use super::atmosphere_pass_tally::大気LUT生成パス数の記録;
use super::cpu_timing::{CPU区間時間, CPU区間計測};
use super::draw_issue_breakdown::描画発行内訳;
use super::レンダラー;
use crate::gpu_memory_stats::GPUメモリ統計;
use crate::present_display_observation::実表示観測;
use crate::present_display_status::実表示計測状況;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

impl レンダラー {
    /// ウォームアップ後に収集したレンダラーCPU区間時間を返す。
    pub fn cpu区間時間一覧を取得する(&self) -> &[CPU区間時間] {
        self.cpu区間計測.as_ref().map_or(&[], CPU区間計測::時間一覧)
    }

    /// 実表示時刻計測の対応状況(拡張と機能フラグの実測結果)。
    pub fn 実表示計測状況を取得する(&self) -> 実表示計測状況 {
        self.実表示計測.状況()
    }

    /// ウォームアップ後に収集した実表示観測。計測していないなら空配列。
    pub fn 実表示観測一覧を取得する(&self) -> &[実表示観測] {
        self.実表示計測.観測一覧()
    }

    /// 現在までのvalidationエラー・警告合計件数を読めるカウンタを複製して返す。
    /// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断3」。
    /// 読み取りはレンダラー破棄後に行うこと。
    pub fn 検証カウンタを取得する(&self) -> 検証カウンタ {
        self.検証カウンタ.clone()
    }

    /// パス名ごとの移動平均GPU時間(ミリ秒)を返す(判断30)。タイムスタンプ非対応
    /// デバイスでは空配列(計測できていないことの明示。無言の0ミリ秒は返さない)。
    pub fn パス別gpu時間を取得する(&self) -> Vec<(&'static str, f64)> {
        self.gpu計測
            .as_ref()
            .map(vulkan::gpu_timing::パス別GPU計測::平均一覧を取得する)
            .unwrap_or_default()
    }

    /// 現在のVulkan専用メモリ確保数、最大同時数、デバイス上限、用途別確保量を返す。
    pub fn gpuメモリ統計を取得する(&self) -> GPUメモリ統計 {
        self.環境.device().メモリ統計を取得する()
    }

    /// 直近に積んだ1フレームぶんの描画発行の内訳。作業領域はフレームごとに積み直すため、最後に描いたフレームの実績である。
    pub fn 描画発行内訳を取得する(&self) -> 描画発行内訳 {
        self.シーン描画資源.描画発行内訳を作る()
    }

    /// 大気LUT生成パスの実行数の記録。フレームごとの本数と累計であり、更新判定が意図どおり働いているかを実測で見る。
    pub fn 大気lut生成パス数の記録を取得する(&self) -> &大気LUT生成パス数の記録 {
        &self.大気lut生成計数
    }

    /// 提示へ到達しなかったフレームの累計。提示停止に起因する異常(破棄待ちの滞留・フレームループの空転)を実行中に観測する計器である。
    pub fn 見送りフレーム数を取得する(&self) -> u64 {
        self.フレーム進行.見送りフレーム数()
    }
}
