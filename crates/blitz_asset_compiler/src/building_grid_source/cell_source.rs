//! 升目1つぶんの素データと、その座標の素データ。4つの側面を欄の名前で持ち、床と屋根を枝で持つ。
//!
//! 側面を並びでなく欄の名前で持つのは、並びの何番目が正面かという約束をJSONの読み書きの両側が覚える形が、
//! 正面と背面を取り違えても通るためである。欄の名前は取り違えると読めない。
//!
//! 床と屋根を真偽でなく枝で持つのは、真の側が何を意味するかを綴りが言うためである。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」

use blitz_assembly::{升目の側面, 升目の宣言, 升目の屋根, 升目の床, 升目の座標};
use serde::{Deserialize, Serialize};

use super::error::建物の格子のソースエラー;
use super::opening_source::はめ口ソース;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct 升目の座標ソース {
    pub 横: i32,
    pub 奥: i32,
    pub 階: u32,
}

impl 升目の座標ソース {
    pub(super) fn 領域の座標へ解く(self) -> 升目の座標 {
        升目の座標::生成する(self.横, self.奥, self.階)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum 升目の床ソース {
    張らない,
    張る,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum 升目の屋根ソース {
    載せない,
    載せる,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct 升目ソース {
    pub 座標: 升目の座標ソース,
    pub 正面: はめ口ソース,
    pub 背面: はめ口ソース,
    pub 左面: はめ口ソース,
    pub 右面: はめ口ソース,
    pub 床: 升目の床ソース,
    pub 屋根: 升目の屋根ソース,
}

impl 升目ソース {
    pub(super) fn 領域の宣言へ解く(self, 建物定義: &str) -> Result<升目の宣言, 建物の格子のソースエラー> {
        let mut 宣言 = 升目の宣言::骨格だけを置く()
            .床を定める(match self.床 {
                升目の床ソース::張らない => 升目の床::張らない,
                升目の床ソース::張る => 升目の床::張る,
            })
            .屋根を定める(match self.屋根 {
                升目の屋根ソース::載せない => 升目の屋根::載せない,
                升目の屋根ソース::載せる => 升目の屋根::載せる,
            });
        for 側面 in 升目の側面::全側面を数え上げる() {
            宣言 = 宣言.側面を定める(側面, self.側面の素データ(側面).領域の宣言へ解く(建物定義)?);
        }
        Ok(宣言)
    }

    fn 側面の素データ(self, 側面: 升目の側面) -> はめ口ソース {
        match 側面 {
            升目の側面::正面 => self.正面,
            升目の側面::背面 => self.背面,
            升目の側面::左面 => self.左面,
            升目の側面::右面 => self.右面,
        }
    }
}
