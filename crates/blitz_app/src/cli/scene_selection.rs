//! 起動時に読むシーンの識別。担当するのは「`--scene`の綴りを起動の入口で1回だけ世界の種別へ解き、
//! 以降の方針の引き当てを網羅`match`で書けるようにする」ことである。綴りの正本と見分けの工程は`parse`が、
//! 種別の数え上げは`world_kind`が、ホットリロード検収の差し替えの対は`rewrite_pair`が持つ。
//!
//! 綴りを入口で1回だけ解くのは、同じ名前の比較が方針ごとに散ると、世界を1つ足したときに
//! どの方針が判断を求められているのかを機械が指摘できなくなるためである。

mod parse;
mod rewrite_pair;
#[cfg(test)]
mod scene_selection_tests;
mod world_kind;

pub(crate) use parse::{地形の夜灯り世界の綴り, 石の小屋の屋内の小物世界の綴り};
pub(crate) use rewrite_pair::差し替える生成物の対;
pub(crate) use world_kind::{世界の種別, 地形世界の種別, 小物世界の種別, 植生の検収世界の種別};

use blitz_engine::アセットID;

use super::起動引数エラー;

/// 起動時に読むシーン。安定idは実行時アセットの論理名そのものであり、どの世界かの判別は種別が持つ。
pub(crate) struct 起動時シーン {
    安定id: アセットID,
    種別: 世界の種別,
}

impl 起動時シーン {
    /// `--scene`の綴りから起動時シーンを解く。空の綴りは安定idになれないため、ここで型付きの失敗になる。
    pub(crate) fn 綴りから解析する(綴り: &str) -> Result<Self, 起動引数エラー> {
        let 安定id = アセットID::生成する(綴り).map_err(|誤り| 起動引数エラー::シーン名不正(誤り.to_string()))?;
        Ok(Self {
            安定id,
            種別: parse::綴りから世界の種別を見分ける(綴り),
        })
    }

    /// `--scene`の指定が無い起動が読むシーン。
    pub(crate) fn 既定() -> Self {
        Self::綴りから解析する(parse::平面板の綴り).unwrap_or_else(|誤り| panic!("既定のシーンの綴りの定数が不正だった: {誤り}"))
    }

    pub(crate) fn 安定id(&self) -> &アセットID {
        &self.安定id
    }

    pub(crate) fn 種別(&self) -> 世界の種別 {
        self.種別
    }

    /// ホットリロードの検収がこの世界で上書きする生成物の対。
    pub(crate) fn 差し替える生成物の対(&self) -> 差し替える生成物の対 {
        差し替える生成物の対::世界の種別から選ぶ(self.種別)
    }
}
