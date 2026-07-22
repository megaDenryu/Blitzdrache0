//! M11のCPU側フレーム間隔分布。FIFO提示待ちを含むフレーム開始間隔を記録する。

#[cfg(test)]
mod frame_timing_tests;

use std::time::Instant;

use crate::cli::起動モード;

const ウォームアップフレーム数: u32 = 120;
const 突発遅延境界MS: f64 = 25.0;

pub(super) struct フレーム間隔計測 {
    前回開始: Option<Instant>,
    呼出回数: u32,
    間隔一覧ms: Vec<f64>,
}

pub(crate) struct フレーム時間統計 {
    pub(crate) 標本数: usize,
    pub(crate) 平均ms: f64,
    pub(crate) p50ms: f64,
    pub(crate) p95ms: f64,
    pub(crate) p99ms: f64,
    pub(crate) 最大ms: f64,
    pub(crate) 二十五ms超過数: usize,
}

impl フレーム間隔計測 {
    pub(super) fn 生成する(モード: 起動モード) -> Self {
        let 容量 = match モード {
            起動モード::スモーク実行 { フレーム数 } => フレーム数.saturating_sub(ウォームアップフレーム数),
            起動モード::無期限実行 => 0,
        };
        Self {
            前回開始: None,
            呼出回数: 0,
            間隔一覧ms: Vec::with_capacity(usize::try_from(容量).unwrap_or_else(|_| panic!("フレーム数がusizeに収まらない"))),
        }
    }

    pub(super) fn 記録する(&mut self) {
        let 今 = Instant::now();
        if let Some(前回開始) = self.前回開始
            && self.呼出回数 >= ウォームアップフレーム数
        {
            self.間隔一覧ms.push(今.duration_since(前回開始).as_secs_f64() * 1000.0);
        }
        self.前回開始 = Some(今);
        self.呼出回数 = self.呼出回数.saturating_add(1);
    }

    pub(crate) fn 集計する(&self) -> Option<フレーム時間統計> {
        集計する(&self.間隔一覧ms)
    }
}

fn 集計する(間隔一覧ms: &[f64]) -> Option<フレーム時間統計> {
    if 間隔一覧ms.is_empty() {
        return None;
    }
    let mut 昇順 = 間隔一覧ms.to_vec();
    昇順.sort_by(f64::total_cmp);
    let (合計, 件数) = 昇順.iter().fold((0.0, 0.0), |(合計, 件数), &値| (合計 + 値, 件数 + 1.0));
    Some(フレーム時間統計 {
        標本数: 昇順.len(),
        平均ms: 合計 / 件数,
        p50ms: 百分位値(&昇順, 50),
        p95ms: 百分位値(&昇順, 95),
        p99ms: 百分位値(&昇順, 99),
        最大ms: 昇順[昇順.len() - 1],
        二十五ms超過数: 昇順.iter().filter(|&&値| 値 > 突発遅延境界MS).count(),
    })
}

fn 百分位値(昇順: &[f64], 百分位: usize) -> f64 {
    let 添字 = 昇順.len().saturating_mul(百分位).div_ceil(100).saturating_sub(1);
    昇順[添字]
}
