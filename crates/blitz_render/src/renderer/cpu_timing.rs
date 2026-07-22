//! ベンチ時だけ有効にするレンダラーCPU区間計測。Vulkan呼び出しの待機位置を切り分ける。

#[cfg(test)]
mod cpu_timing_tests;

use std::time::Instant;

/// 1フレームのレンダラーCPU区間時間。終了時の診断ログへ渡す安全な計測値である。
#[derive(Clone, Copy)]
pub struct CPU区間時間 {
    pub フェンス待機ms: f64,
    pub フレームデータ準備ms: f64,
    pub 画像取得ms: f64,
    pub 記録送信提示ms: f64,
}

pub(super) struct CPU区間計測 {
    ウォームアップフレーム数: u32,
    呼出回数: u32,
    時間一覧: Vec<CPU区間時間>,
}

pub(super) struct CPU区間時計 {
    開始: Instant,
    フェンス待機終了: Instant,
    フレームデータ準備終了: Instant,
    画像取得終了: Instant,
}

impl CPU区間計測 {
    pub(super) fn 生成する(ウォームアップフレーム数: u32, 標本容量: usize) -> Self {
        Self {
            ウォームアップフレーム数,
            呼出回数: 0,
            時間一覧: Vec::with_capacity(標本容量),
        }
    }

    pub(super) fn 記録する(&mut self, 時間: CPU区間時間) {
        if self.呼出回数 >= self.ウォームアップフレーム数 {
            self.時間一覧.push(時間);
        }
        self.呼出回数 = self.呼出回数.saturating_add(1);
    }

    pub(super) fn 時間一覧(&self) -> &[CPU区間時間] {
        &self.時間一覧
    }
}

impl CPU区間時計 {
    pub(super) fn 開始する() -> Self {
        let 現在 = Instant::now();
        Self {
            開始: 現在,
            フェンス待機終了: 現在,
            フレームデータ準備終了: 現在,
            画像取得終了: 現在,
        }
    }

    pub(super) fn フェンス待機を終了する(&mut self) {
        self.フェンス待機終了 = Instant::now();
    }

    pub(super) fn フレームデータ準備を終了する(&mut self) {
        self.フレームデータ準備終了 = Instant::now();
    }

    pub(super) fn 画像取得を終了する(&mut self) {
        self.画像取得終了 = Instant::now();
    }

    pub(super) fn 終了する(self) -> CPU区間時間 {
        let 終了 = Instant::now();
        CPU区間時間 {
            フェンス待機ms: self.フェンス待機終了.duration_since(self.開始).as_secs_f64() * 1000.0,
            フレームデータ準備ms: self.フレームデータ準備終了.duration_since(self.フェンス待機終了).as_secs_f64() * 1000.0,
            画像取得ms: self.画像取得終了.duration_since(self.フレームデータ準備終了).as_secs_f64() * 1000.0,
            記録送信提示ms: 終了.duration_since(self.画像取得終了).as_secs_f64() * 1000.0,
        }
    }
}
