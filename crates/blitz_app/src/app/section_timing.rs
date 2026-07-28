//! 1つのCPU区間がメインスレッドを占めた時間の分布。担当するのは標本の列だけであり、どの区間を測るかは呼び出し側が決める。
//! 先頭のウォームアップフレームを除く規則と分位の求め方はフレーム間隔の計測とそろえ、同じ実行の値を並べて読めるようにする。

use std::time::Duration;

use super::frame_timing::{ウォームアップフレーム数, フレーム時間統計, 標本容量, 集計する};
use crate::cli::起動モード;

pub(crate) struct 区間計測 {
    呼出回数: u32,
    時間一覧ms: Vec<f64>,
}

impl 区間計測 {
    pub(crate) fn 生成する(モード: 起動モード) -> Self {
        Self {
            呼出回数: 0,
            時間一覧ms: Vec::with_capacity(標本容量(モード)),
        }
    }

    pub(crate) fn 記録する(&mut self, 所要時間: Duration) {
        if self.呼出回数 >= ウォームアップフレーム数 {
            self.時間一覧ms.push(所要時間.as_secs_f64() * 1000.0);
        }
        self.呼出回数 = self.呼出回数.saturating_add(1);
    }

    pub(crate) fn 集計する(&self) -> Option<フレーム時間統計> {
        集計する(&self.時間一覧ms)
    }
}
