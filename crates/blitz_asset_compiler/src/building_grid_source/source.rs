//! 建物の格子の保存物の最新の形。トップレベルを「格子」と「装飾」の2区画に分け、格子の区画だけが
//! 組み立ての検査に掛かる。
//!
//! 装飾の区画を中身が無いうちから切っておくのは、検査に乗らない自由配置を後から足すときに、トップレベルの
//! 欄の並びを組み替えずに済ませるためである。区画を後から足すと、既存の保存物が全部旧版になる。
//! 参照: `_doc/計画/エディターからゲームまでの統合の作戦.md`「決定3」

use blitz_assembly::ベイ格子の組み手;
use serde::{Deserialize, Serialize};

use super::cell_source::{升目の座標ソース, 升目ソース};
use super::error::建物の格子のソースエラー;
use super::grid_definition::格子由来の建物定義;
use crate::runtime_compilation::{建物の入口方向, 建物定義ID};

pub const 建物の格子ソースの現在の形式版: u32 = 1;

/// 検査に乗らない自由配置の飾りを収める先。初版は欄を1つも持たない。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct 建物の装飾ソース {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct 建物の入口方向ソース {
    pub x: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct 升目の複体ソース {
    pub 根の升目: 升目の座標ソース,
    pub 升目一覧: Vec<升目ソース>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct 建物の格子ソース {
    pub 形式版: u32,
    pub 建物定義ID: 建物定義ID,
    pub 表示名: String,
    pub 入口のローカル方向: 建物の入口方向ソース,
    pub 格子: 升目の複体ソース,
    pub 装飾: 建物の装飾ソース,
}

impl 建物の格子ソース {
    /// 升目を昇順に置き、名指した根で閉じる。升目の宣言の破れと到達の破れはベイ格子の組み手が拒む。
    pub fn 格子由来の建物定義へ解く(&self) -> Result<格子由来の建物定義, 建物の格子のソースエラー> {
        let 建物定義 = self.建物定義ID.綴り();
        let mut 組み手 = ベイ格子の組み手::空から始める();
        for 升目 in &self.格子.升目一覧 {
            let 宣言 = 升目.領域の宣言へ解く(建物定義)?;
            組み手
                .升目を1つ置く(升目.座標.領域の座標へ解く(), 宣言)
                .map_err(|原因| self.升目が成り立たないエラーを作る(原因))?;
        }
        let 格子 = 組み手
            .根を定めて閉じる(self.格子.根の升目.領域の座標へ解く())
            .map_err(|原因| self.升目が成り立たないエラーを作る(原因))?;
        Ok(格子由来の建物定義::解けた材料から生成する(
            self.建物定義ID.clone(),
            self.表示名.clone(),
            建物の入口方向 {
                x: self.入口のローカル方向.x,
                z: self.入口のローカル方向.z,
            },
            格子,
        ))
    }

    fn 升目が成り立たないエラーを作る(&self, 原因: blitz_assembly::ベイ格子エラー) -> 建物の格子のソースエラー {
        建物の格子のソースエラー::升目が成り立たない {
            建物定義: self.建物定義ID.綴り().to_string(),
            原因,
        }
    }
}
