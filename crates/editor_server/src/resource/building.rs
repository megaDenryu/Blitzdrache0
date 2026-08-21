//! 建物の配置の型契約。チャンクの編集資源であり、建物外形カタログの定義1件を
//! 位置・向き・基礎半径・なじみ半径とともに表す。基礎半径は地形を平坦化する範囲、
//! なじみ半径はその外側で元の地形へ滑らかに戻す範囲であり、モックアップの
//! `footprintRadius`/`blendRadius`に対応する。

use serde::{Deserialize, Serialize};

use super::building_definition_id::建物定義ID;
use super::numeric_check::{有限であることを確かめる, 正の有限数であることを確かめる};
use super::position::位置3次元;
use super::validation_error::資源検証エラー;

/// 建物の配置とは、1件の建物をチャンクへ据える指定のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[allow(non_snake_case)]
pub struct 建物の配置 {
    pub 識別子: String,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 建物定義ID: 建物定義ID,
    pub 位置: 位置3次元,
    pub 向きラジアン: f64,
    pub 基礎半径メートル: f64,
    pub なじみ半径メートル: f64,
}

impl 建物の配置 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if self.識別子.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        self.位置.検証する()?;
        有限であることを確かめる("建物の配置.向きラジアン", self.向きラジアン)?;
        正の有限数であることを確かめる("建物の配置.基礎半径メートル", self.基礎半径メートル)?;
        正の有限数であることを確かめる("建物の配置.なじみ半径メートル", self.なじみ半径メートル)?;
        if self.なじみ半径メートル < self.基礎半径メートル {
            return Err(資源検証エラー::なじみ半径が基礎半径を下回る {
                基礎半径: self.基礎半径メートル,
                なじみ半径: self.なじみ半径メートル,
            });
        }
        Ok(())
    }
}
