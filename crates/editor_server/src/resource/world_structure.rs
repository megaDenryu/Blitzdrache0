//! 大域世界構造の型契約。`GET|PUT /api/大域世界/構造`が読み書きするJSONの正本であり、
//! 区画割りと広域道路を1つの直列化単位へ束ねる(高さ格子は別口のバイナリ資源)。

use serde::{Deserialize, Serialize};

use super::regional_road::広域道路;
use super::validation_error::資源検証エラー;
use super::world_layout::世界の区画割り;

/// 大域世界構造とは、大域世界の区画割りと広域道路をまとめた、`大域世界/構造`の
/// JSON1本ぶんの内容のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 大域世界構造 {
    pub 区画割り: 世界の区画割り,
    pub 広域道路: 広域道路,
}

impl 大域世界構造 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        self.区画割り.検証する()?;
        self.広域道路.検証する()?;
        Ok(())
    }
}
