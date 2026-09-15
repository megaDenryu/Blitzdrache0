//! M11のCPU側フレーム間隔分布。FIFO提示待ちを含むフレーム開始間隔を記録する。

#[cfg(test)]
mod frame_timing_tests;

use std::time::Instant;

use blitz_math::{パーセンタイル順位, 昇順に並べた標本列, 順位から添字を選ぶ規則};

use crate::cli::起動モード;

pub(crate) const ウォームアップフレーム数: u32 = 120;
const 突発遅延境界MS: f64 = 25.0;
/// この統計が添字を選ぶ規則。寄せる前からこの規則であり、報告の値を動かさないために変えない。
const 添字を選ぶ規則: 順位から添字を選ぶ規則 = 順位から添字を選ぶ規則::件数を基準にした切り上げの1つ手前;

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
        起動モード::段差走査実行 { 走査指定 } => 走査指定.描画機会の総数().saturating_sub(ウォームアップフレーム数),
        起動モード::スモーク実行 { 描画機会の数 } | 起動モード::ベンチ実行 { 描画機会の数 } => 描画機会の数.saturating_sub(ウォームアップフレーム数),
        起動モード::無期限実行 => 0,
    };
    usize::try_from(容量).unwrap_or_else(|_| panic!("フレーム数がusizeに収まらない"))
}

pub(crate) fn フレーム間隔から統計を集計する(間隔一覧ms: &[f64]) -> Option<フレーム時間統計> {
    if 間隔一覧ms.is_empty() {
        return None;
    }
    let mut 値の写し = 間隔一覧ms.to_vec();
    let 標本列 = 昇順に並べた標本列::並べ替えて生成する(&mut 値の写し);
    let (合計, 件数) = 標本列.昇順の値一覧().iter().fold((0.0, 0.0), |(合計, 件数), &値| (合計 + 値, 件数 + 1.0));
    Some(フレーム時間統計 {
        標本数: 標本列.件数(),
        平均ミリ秒: 合計 / 件数,
        五十パーセンタイル値ミリ秒: 百分位値(&標本列, 50.0)?,
        九十五パーセンタイル値ミリ秒: 百分位値(&標本列, 95.0)?,
        九十九パーセンタイル値ミリ秒: 百分位値(&標本列, 99.0)?,
        最大ミリ秒: 標本列.最大()?,
        二十五ミリ秒超過数: 標本列.昇順の値一覧().iter().filter(|&&値| 値 > 突発遅延境界MS).count(),
    })
}

fn 百分位値(標本列: &昇順に並べた標本列<'_, f64>, 順位: f32) -> Option<f64> {
    標本列.パーセンタイル値を求める(パーセンタイル順位::百分率から生成する(順位), 添字を選ぶ規則)
}
