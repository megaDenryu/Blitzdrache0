//! 建物外形カタログのHTTP契約。版付きJSONの読み込みと検証は子モジュールへ閉じる。

mod loading;
mod validation;

use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

pub use loading::建物外形カタログ読み込みエラー;

pub const 建物外形カタログの現在の形式版: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub enum 建物定義の用途 {
    骨格見本,
    家屋,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct ベイ構造 {
    pub 横: u32,
    pub 奥: u32,
    pub 階: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物の入口方向 {
    pub x: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物の外接箱 {
    pub 最小: [f32; 3],
    pub 最大: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物外形定義 {
    pub 識別子: String,
    pub 表示名: String,
    pub 用途: 建物定義の用途,
    pub 部品の識別子一覧: Vec<String>,
    pub ベイ: ベイ構造,
    pub 高さメートル: f32,
    pub 入口のローカル方向: 建物の入口方向,
    pub 外接箱: 建物の外接箱,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物外形カタログ {
    pub 形式版: u32,
    pub 建物定義一覧: Vec<建物外形定義>,
}

impl 建物外形カタログ {
    pub fn 建物定義を含む(&self, 識別子: &str) -> bool {
        self.建物定義一覧.iter().any(|定義| 定義.識別子 == 識別子)
    }
}
