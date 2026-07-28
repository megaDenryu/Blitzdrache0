//! 毎フレームの座標別LODを前回値と比較し、段の変更回数と同じフレームに始まったディスク読込を集計する。

use blitz_engine::{ストリーミング進行, チャンク座標};
use blitz_render::地形詳細段;

pub(super) struct LOD変更計測 {
    前回: Vec<(チャンク座標, 地形詳細段)>,
    変更フレーム数: u32,
    変更時読込開始件数: u64,
}

impl LOD変更計測 {
    pub(super) fn 生成する() -> Self {
        Self {
            前回: Vec::new(),
            変更フレーム数: 0,
            変更時読込開始件数: 0,
        }
    }

    pub(super) fn 記録する(&mut self, 進行: &ストリーミング進行, 現在: &[(チャンク座標, 地形詳細段)]) {
        let 変更 = !self.前回.is_empty()
            && 現在
                .iter()
                .any(|(座標, 段)| self.前回.iter().any(|(前座標, 前段)| 座標 == 前座標 && 段 != 前段));
        if 変更 {
            self.変更フレーム数 = self.変更フレーム数.saturating_add(1);
            self.変更時読込開始件数 = self
                .変更時読込開始件数
                .saturating_add(u64::try_from(進行.読込開始一覧().len()).unwrap_or(u64::MAX));
        }
        self.前回.clear();
        self.前回.extend_from_slice(現在);
    }

    pub(super) fn 変更フレーム数(&self) -> u32 {
        self.変更フレーム数
    }

    pub(super) fn 変更時読込開始件数(&self) -> u64 {
        self.変更時読込開始件数
    }
}
