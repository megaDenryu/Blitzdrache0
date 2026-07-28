//! 常駐チャンクの詳細段を毎フレーム決める選択器。距離・ヒステリシス・隣接差の緩和の3工程を順に呼び、前回の段だけを状態として持つ。
//! 保持する前回の段は距離とヒステリシスだけで決まる値であり、緩和の結果は残さない。緩和は隣接の常駐状況で毎フレーム変わる制約であり、
//! 結果を次フレームの出発点にすると、隣接が解除された後も引き下げが残り続けるためである。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

use std::collections::HashMap;

use blitz_math::{大域メートル, 大域ワールド位置};
use blitz_render::地形詳細段;

use super::distance::チャンクとの最短距離;
use super::error::地形LODエラー;
use super::hysteresis::段を決める;
use super::relaxation::隣接差を緩和する;
use super::selection_settings::地形LOD選択設定;
use crate::チャンク座標;

pub struct 地形LOD選択器 {
    設定: 地形LOD選択設定,
    一辺: 大域メートル,
    前回の段: HashMap<チャンク座標, 地形詳細段>,
}

impl 地形LOD選択器 {
    pub fn 生成する(設定: 地形LOD選択設定, 一辺: 大域メートル) -> Result<Self, 地形LODエラー> {
        if !一辺.値().is_finite() || 一辺.値() <= 0.0 {
            return Err(地形LODエラー::一辺不正);
        }
        Ok(Self {
            設定,
            一辺,
            前回の段: HashMap::new(),
        })
    }

    /// 常駐座標一覧から段を決めて受け皿へ書く。受け皿を使い回す形にするのは、毎フレームの選択でヒープを再確保しないためである。
    /// 受け皿は座標の辞書順に並ぶ。緩和の走査順がこの並びで決まるため、同じ常駐集合と同じ視点から常に同じ結果を得る。
    pub fn 選択する(
        &mut self, 視点: 大域ワールド位置, 常駐座標一覧: &[チャンク座標], 受け皿: &mut Vec<(チャンク座標, 地形詳細段)>
    ) {
        受け皿.clear();
        for 座標 in 常駐座標一覧 {
            let 距離 = チャンクとの最短距離(視点, *座標, self.一辺);
            受け皿.push((*座標, 段を決める(self.前回の段.get(座標).copied(), 距離, self.設定)));
        }
        受け皿.sort_by_key(|(座標, _)| (座標.x(), 座標.z()));
        self.前回の段.clear();
        self.前回の段.extend(受け皿.iter().copied());
        隣接差を緩和する(受け皿);
    }
}
