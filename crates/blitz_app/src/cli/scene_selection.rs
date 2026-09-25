//! 起動時に読むシーンの識別。担当するのは「`--scene`の文字列を起動の入口で1回だけ世界の種別へ解き、
//! 以降の方針の引き当てを網羅`match`で書けるようにする」ことである。文字列の正本と見分けの工程は`parse`が、
//! 種別の数え上げは`world_kind`が、文字列の正本は`scene_name`が、ホットリロード検収の差し替えの対は`rewrite_pair`が持つ。
//!
//! 文字列を入口で1回だけ解くのは、同じ名前の比較が方針ごとに散ると、世界を1つ足したときに
//! どの方針が判断を求められているのかを機械が指摘できなくなるためである。

mod parse;
mod rewrite_pair;
mod scene_name;
#[cfg(test)]
mod scene_selection_tests;
mod world_kind;

pub(crate) use rewrite_pair::差し替える生成物の対;
pub(crate) use scene_name::{地形の夜灯り世界のシーン名, 石の小屋の屋内の小物世界のシーン名};
pub(crate) use world_kind::{世界の種別, 地形世界の種別, 小物世界の種別, 植生の検収世界の種別};

use blitz_engine::アセットID;

use super::起動引数エラー;

/// 起動時に読むシーン。安定idは実行時アセットの論理名そのものであり、どの世界かの判別は種別が持つ。
pub(crate) struct 起動時シーン {
    安定id: アセットID,
    種別: 世界の種別,
}

impl 起動時シーン {
    /// `--scene`の文字列から起動時シーンを解く。空の文字列は安定idになれないため、ここで型付きの失敗になる。
    pub(crate) fn 文字列から解析する(文字列: &str) -> Result<Self, 起動引数エラー> {
        let 安定id = アセットID::生成する(文字列).map_err(|誤り| 起動引数エラー::シーン名不正(誤り.to_string()))?;
        Ok(Self {
            安定id,
            種別: parse::文字列から世界の種別を見分ける(文字列),
        })
    }

    /// `--scene`の指定が無い起動が読むシーン。
    pub(crate) fn 既定() -> Self {
        Self::文字列から解析する(scene_name::平面板のシーン名).unwrap_or_else(|誤り| panic!("既定のシーンの文字列の定数が不正だった: {誤り}"))
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
