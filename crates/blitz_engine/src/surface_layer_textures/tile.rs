//! 地表層のタイル1件。担当するのは、エディターが付けたエンジン材質名と、その材質のタイルテクスチャを
//! 1つの組として保つことである。
//!
//! 名前とテクスチャを別々の並びで持たないのは、片方だけを並べ替えたときに材質と絵が入れ替わっても型が通るためである。

use crate::asset::texture_storage::格納済みテクスチャ;

use super::error::地表層テクスチャ集エラー;

#[derive(Debug, Clone, PartialEq)]
pub struct 地表層のタイル {
    材質名: String,
    ベースカラー: 格納済みテクスチャ,
}

impl 地表層のタイル {
    pub fn 生成する(材質名: String, ベースカラー: 格納済みテクスチャ) -> Result<Self, 地表層テクスチャ集エラー> {
        if 材質名.trim().is_empty() {
            return Err(地表層テクスチャ集エラー::材質名が空);
        }
        Ok(Self {
            材質名, ベースカラー
        })
    }

    pub fn 材質名(&self) -> &str {
        &self.材質名
    }

    pub fn ベースカラー(&self) -> &格納済みテクスチャ {
        &self.ベースカラー
    }
}
