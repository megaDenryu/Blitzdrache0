//! 編集画面と建物の焼き込みが共有する、版付きの建物外形カタログ。

use serde::Serialize;

use super::building_definition_id::建物定義ID;

pub const 現在の形式版: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum 建物定義の用途 {
    骨格見本,
    家屋,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ベイ構造 {
    pub 横: u32,
    pub 奥: u32,
    pub 階: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct 建物の入口方向 {
    pub x: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct 建物の外接箱 {
    pub 最小: [f32; 3],
    pub 最大: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct 建物定義 {
    pub 識別子: 建物定義ID,
    pub 表示名: String,
    pub 用途: 建物定義の用途,
    pub 部品の識別子一覧: Vec<String>,
    pub ベイ: ベイ構造,
    pub 入口のローカル方向: 建物の入口方向,
    pub 外接箱: 建物の外接箱,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct 建物外形カタログ {
    pub 形式版: u32,
    pub 建物定義一覧: Vec<建物定義>,
}

impl 建物外形カタログ {
    pub(super) fn 定義一覧から生成する(建物定義一覧: Vec<建物定義>) -> Self {
        Self {
            形式版: 現在の形式版,
            建物定義一覧,
        }
    }
}
