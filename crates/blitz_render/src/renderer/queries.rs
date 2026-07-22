//! レンダラーの読み取り系アクセサと寸法変更通知。`mod.rs`の行数分割のための切り出し。

use super::cpu_timing::{CPU区間時間, CPU区間計測};
use super::レンダラー;
use crate::extent::ウィンドウ寸法;
use crate::gpu_memory_stats::GPUメモリ統計;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

impl レンダラー {
    /// ベンチ用CPU区間計測を有効にする。通常実行では計測時計を読まない。
    pub fn cpu区間計測を有効にする(&mut self, ウォームアップフレーム数: u32, 標本容量: usize) {
        self.cpu区間計測 = Some(CPU区間計測::生成する(ウォームアップフレーム数, 標本容量));
    }

    /// ウォームアップ後に収集したレンダラーCPU区間時間を返す。
    pub fn cpu区間時間一覧を取得する(&self) -> &[CPU区間時間] {
        self.cpu区間計測.as_ref().map_or(&[], CPU区間計測::時間一覧)
    }

    /// 現在までのvalidationエラー・警告合計件数を読めるカウンタを複製して返す。
    /// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断3」。
    /// 読み取りはレンダラー破棄後に行うこと。
    pub fn 検証カウンタを取得する(&self) -> 検証カウンタ {
        self.検証カウンタ.clone()
    }

    /// ウィンドウの寸法変更を通知する。次フレームでスワップチェーンを再構築する。
    pub fn サイズ変更を通知する(&mut self, 寸法: ウィンドウ寸法) {
        self.現在の寸法 = 寸法;
        self.再構築が必要 = true;
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
        self.device.メモリ統計を取得する()
    }
}
