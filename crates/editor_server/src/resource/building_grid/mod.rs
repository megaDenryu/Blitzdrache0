//! 建物の格子の型契約。建物エディターが編集する升目の複体を、素データ様式でブラウザとサーバーが共有する。
//!
//! トップレベルを「格子」と「装飾」の2区画に切るのは、検査に乗らない自由配置の飾りを後から足すときに、
//! 欄の並びを組み替えずに済ませるためである。装飾の区画は初版が欄を1つも持たない。
//!
//! 形式版の写しがアセットコンパイラ側(`building_grid_source/source.rs`)にもある。両者が食い違うと
//! 読みが必ず「形式版に対応していない」で落ちる。一致は`cargo xtask conform`の定数の組が見る。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」

mod cell;
mod listing;
mod opening;
mod source_conversion;

use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

use super::building_definition_id::建物定義ID;
use super::building_outline_catalog::建物の入口方向;
use super::numeric_check::有限であることを確かめる;
use super::validation_error::資源検証エラー;

pub use cell::{升目の宣言, 升目の屋根, 升目の床, 升目の座標};
pub use listing::建物の格子の一覧項目;
pub use opening::{はめ口の値, 壁の外面の飾り, 壁の種類};

pub const 建物の格子の現在の形式版: u32 = 1;

/// 検査に乗らない自由配置の飾りを収める区画。初版は欄を1つも持たない。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物の格子の装飾 {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 升目の複体 {
    pub 根の升目: 升目の座標,
    pub 升目一覧: Vec<升目の宣言>,
}

/// 建物の格子とは、建物定義1件を升目の複体として表した編集資源のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[allow(non_snake_case)]
pub struct 建物の格子 {
    pub 形式版: u32,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 建物定義ID: 建物定義ID,
    pub 表示名: String,
    pub 入口のローカル方向: 建物の入口方向,
    pub 格子: 升目の複体,
    pub 装飾: 建物の格子の装飾,
}

impl 建物の格子 {
    /// 形式版と表示名と入口方向の欄そのものを見る。升目の組み合わせが成り立つかは、
    /// アセットコンパイラのベイ格子への解決が受け持つ(検査の正本を2つ持たない)。
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if self.形式版 != 建物の格子の現在の形式版 {
            return Err(資源検証エラー::参照先が存在しない {
                フィールド名: "建物の格子.形式版",
                値: self.形式版.to_string(),
            });
        }
        if self.表示名.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        有限であることを確かめる("建物の格子.入口のローカル方向.x", f64::from(self.入口のローカル方向.x))?;
        有限であることを確かめる("建物の格子.入口のローカル方向.z", f64::from(self.入口のローカル方向.z))?;
        Ok(())
    }
}
