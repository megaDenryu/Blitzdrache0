//! CPU側フレーム時間の移動平均(指数移動平均、判断34)。stdのみで完結する。

use std::time::Instant;

const 平滑化係数: f64 = 0.1;

pub(crate) struct フレーム時間計測 {
    直前時刻: Instant,
    移動平均ms: f64,
}

impl フレーム時間計測 {
    pub(crate) fn 生成する() -> Self {
        Self { 直前時刻: Instant::now(), 移動平均ms: 0.0 }
    }

    /// 前回呼び出しからの経過時間を反映し、更新後の移動平均(ミリ秒)を返す。
    pub(crate) fn 記録する(&mut self) -> f64 {
        let 今 = Instant::now();
        let 経過ms = 今.duration_since(self.直前時刻).as_secs_f64() * 1000.0;
        self.直前時刻 = 今;
        self.移動平均ms += (経過ms - self.移動平均ms) * 平滑化係数;
        self.移動平均ms
    }
}
