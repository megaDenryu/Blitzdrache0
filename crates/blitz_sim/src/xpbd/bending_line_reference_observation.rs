//! 留めた線の参照計算の観測の口: 先端の位置・各節の折れ角・隣り合う点の最大の伸び。触れるのは位置一覧だけであり、刻みを進める側(`bending_line_reference_harness`)と分ける。

use blitz_math::{メートル, ワールド, 位置};

use super::bending_line_projection::線の折れ角の幾何;
use super::bending_line_reference_harness::留めた線の参照計算;

impl 留めた線の参照計算 {
    /// 先端の位置。
    pub(super) fn 先端(&self) -> 位置<ワールド> {
        self.位置一覧[self.位置一覧.len() - 1]
    }

    /// 各節の折れ角(ラジアン)。一直線上の節は0である。
    pub(super) fn 折れ角一覧(&self) -> Vec<f32> {
        (0..self.位置一覧.len().saturating_sub(2))
            .map(|番号| {
                match 線の折れ角の幾何::位置から測る(self.位置一覧[番号], self.位置一覧[番号 + 1], self.位置一覧[番号 + 2])
                {
                    Ok(幾何) => 幾何.折れ角().値(),
                    Err(_) => 0.0,
                }
            })
            .collect()
    }

    /// 隣り合う点の距離の静止長からのずれの最大(伸びの観測)。
    pub(super) fn 最大の伸び(&self, 静止長: メートル) -> f32 {
        self.位置一覧
            .windows(2)
            .map(|対| ((対[1] - 対[0]).長さ() - 静止長).値().abs())
            .fold(0.0, f32::max)
    }
}
