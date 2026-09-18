//! M11のCPU側フレーム間隔分布。FIFO提示待ちを含むフレーム開始間隔を記録する。

#[cfg(test)]
mod frame_timing_tests;

use std::time::Instant;

use crate::cli::起動モード;

pub(crate) const ウォームアップフレーム数: u32 = 120;
const 突発遅延境界MS: f64 = 25.0;

pub(super) struct フレーム間隔計測 {
    前回開始: Option<Instant>,
    呼出回数: u32,
    間隔一覧ms: Vec<f64>,
}

pub(crate) struct フレーム時間統計 {
    pub(crate) 標本数: usize,
    pub(crate) 平均ミリ秒: f64,
    pub(crate) 五十パーセンタイル値ミリ秒: f64,
    pub(crate) 九十五パーセンタイル値ミリ秒: f64,
    pub(crate) 九十九パーセンタイル値ミリ秒: f64,
    pub(crate) 最大ミリ秒: f64,
    pub(crate) 二十五ミリ秒超過数: usize,
}

impl フレーム間隔計測 {
    pub(super) fn 生成する(モード: 起動モード) -> Self {
        Self {
            前回開始: None,
            呼出回数: 0,
            間隔一覧ms: Vec::with_capacity(標本容量(モード)),
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
        フレーム間隔から統計を集計する(&self.間隔一覧ms)
    }
}

pub(crate) fn 標本容量(モード: 起動モード) -> usize {
    let 容量 = match モード {
        起動モード::段差走査実行 { 走査指定 } => 走査指定.総フレーム数().saturating_sub(ウォームアップフレーム数),
        起動モード::スモーク実行 { 描画機会の数 } | 起動モード::ベンチ実行 { 描画機会の数 } => 描画機会の数.saturating_sub(ウォームアップフレーム数),
        起動モード::無期限実行 => 0,
    };
    usize::try_from(容量).unwrap_or_else(|_| panic!("フレーム数がusizeに収まらない"))
}

pub(crate) fn フレーム間隔から統計を集計する(間隔一覧ms: &[f64]) -> Option<フレーム時間統計> {
    if 間隔一覧ms.is_empty() {
        return None;
    }
    let mut 昇順 = 間隔一覧ms.to_vec();
    昇順.sort_by(f64::total_cmp);
    let (合計, 件数) = 昇順.iter().fold((0.0, 0.0), |(合計, 件数), &値| (合計 + 値, 件数 + 1.0));
    Some(フレーム時間統計 {
        標本数: 昇順.len(),
        平均ミリ秒: 合計 / 件数,
        五十パーセンタイル値ミリ秒: 百分位値(&昇順, 50),
        九十五パーセンタイル値ミリ秒: 百分位値(&昇順, 95),
        九十九パーセンタイル値ミリ秒: 百分位値(&昇順, 99),
        最大ミリ秒: 昇順[昇順.len() - 1],
        二十五ミリ秒超過数: 昇順.iter().filter(|&&値| 値 > 突発遅延境界MS).count(),
    })
}

fn 百分位値(昇順: &[f64], 百分位: usize) -> f64 {
    let 添字 = 昇順.len().saturating_mul(百分位).div_ceil(100).saturating_sub(1);
    昇順[添字]
}
