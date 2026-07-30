//! 正規直交な軸3本と軸ごとの区間で表した、方向光の正射影を読めた直方体。触れるフィールドは軸一覧と区間の
//! 2つに限り、中心・半幅はこの2つだけから導く。
//!
//! 全域(正射影を読めなかったときの腕)はこの型を経由しない。中心の一般式は`(最小+最大)*0.5`を軸方向へ
//! 掛け合わせるため、区間に無限大が混じると`無限大 * 0`の項からNaNが生まれる。全域は`光空間区間::全域()`
//! を直接使うことでこの式を回避する(参照: `../ortho_box.rs`の`正射影直方体::全域`)。

use super::super::super::light_space_interval::光空間区間;

pub(in crate::visibility::band_light_box) struct 範囲内直方体 {
    軸一覧: [[f32; 3]; 3],
    区間: 光空間区間,
}

impl 範囲内直方体 {
    pub(in crate::visibility::band_light_box) fn 生成する(軸一覧: [[f32; 3]; 3], 区間: 光空間区間) -> Self {
        Self { 軸一覧, 区間 }
    }

    pub(in crate::visibility::band_light_box) fn 軸(&self, 番号: usize) -> [f32; 3] {
        self.軸一覧[番号]
    }

    /// 直方体の中心(カメラ相対空間)。軸が正規直交であるため、軸ごとの区間の中央を軸方向へ足した点が中心になる。
    pub(in crate::visibility::band_light_box) fn 中心(&self) -> [f32; 3] {
        let 中央 = [0, 1, 2].map(|軸| (self.区間.最小()[軸] + self.区間.最大()[軸]) * 0.5);
        [0, 1, 2].map(|成分| (0..3).map(|軸| 中央[軸] * self.軸一覧[軸][成分]).sum())
    }

    pub(in crate::visibility::band_light_box) fn 半幅(&self) -> [f32; 3] {
        [0, 1, 2].map(|軸| (self.区間.最大()[軸] - self.区間.最小()[軸]) * 0.5)
    }
}
