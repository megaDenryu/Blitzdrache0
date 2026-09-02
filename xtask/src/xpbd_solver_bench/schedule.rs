//! どの条件をどの順で起動するかを決める工程。受け取るのは実行の指定、返すのは周回の位置を付けた条件の並びである。
//!
//! グラフごとに方式をラテン方陣の回転(ABC・BCA・CAB)で3周するのは、周回の中の温まりとドリフトが方式に交絡するためである。
//! 各方式が先頭・2番目・3番目をちょうど1回ずつ取るため、順序の効果が3方式へ等しく配られる。
//! 周回番号と順序位置を生標本へ残すのは、順序の効果が残っていないかを後から読み手が確かめられるようにするためである。

use super::plan::実行の指定;
use crate::acceptance::{検収の実行名, 検収エラー};

/// 比べる並列方式。値はblitz_appの`--xpbd-method`へそのまま渡る語である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 方式 {
    原子加算,
    グラフ彩色,
    二段階,
}

impl 方式 {
    pub(super) const fn 全部() -> &'static [方式] {
        &[Self::原子加算, Self::グラフ彩色, Self::二段階]
    }

    pub(super) const fn 起動指定の語(self) -> &'static str {
        match self {
            Self::原子加算 => "atomic",
            Self::グラフ彩色 => "coloring",
            Self::二段階 => "two-stage",
        }
    }
}

/// 題材の拘束グラフ。値はblitz_appの`--xpbd-graph`へそのまま渡る語である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum グラフ {
    規則格子,
    不規則,
}

impl グラフ {
    pub(super) const fn 全部() -> &'static [グラフ] {
        &[Self::規則格子, Self::不規則]
    }

    pub(super) const fn 起動指定の語(self) -> &'static str {
        match self {
            Self::規則格子 => "grid",
            Self::不規則 => "irregular",
        }
    }
}

/// 測る1条件。方式とグラフの組である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 実行条件 {
    pub(super) 方式: 方式,
    pub(super) グラフ: グラフ,
}

impl 実行条件 {
    /// 表と生値に書く条件の名前。方式とグラフの語をハイフンで結ぶ。
    pub(super) fn 名前(&self) -> String {
        format!("{}-{}", self.方式.起動指定の語(), self.グラフ.起動指定の語())
    }

    pub(super) fn 実行名を組む(&self) -> Result<検収の実行名, 検収エラー> {
        検収の実行名::生成する(&format!("xpbd_{}_{}", self.方式.起動指定の語(), self.グラフ.起動指定の語()))
    }
}

/// 回す周回の数。方式の数と同じにするのは、各方式が全部の順序位置をちょうど1回ずつ取るためである。
pub(super) const 周回数: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 周回の位置 {
    pub(super) 周回番号: usize,
    pub(super) 順序位置: usize,
}

/// 起動する順の並び。周回ごとに、グラフごとの方式の開始位置を1つずらす。
pub(super) fn 起動の並び(指定: &実行の指定) -> Vec<(周回の位置, 実行条件)> {
    let mut 並び = Vec::new();
    for 周回番号 in 0..周回数 {
        for グラフ in &指定.グラフ一覧 {
            for 順序位置 in 0..指定.方式一覧.len() {
                let 方式 = 指定.方式一覧[(周回番号 + 順序位置) % 指定.方式一覧.len()];
                並び.push((
                    周回の位置 { 周回番号, 順序位置 },
                    実行条件 {
                        方式, グラフ: *グラフ
                    },
                ));
            }
        }
    }
    並び
}
