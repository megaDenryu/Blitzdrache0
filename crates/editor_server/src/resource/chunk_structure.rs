//! チャンク構造の型契約。`GET|PUT /api/チャンク/{x}/{z}/構造`が読み書きするJSONの正本であり、
//! 道路・建物一覧・散布の設定を1つの直列化単位へ束ねる(高さ格子・材質重みは別口のバイナリ資源)。

use serde::{Deserialize, Serialize};

use super::building::建物の配置;
use super::chunk_road::チャンクの道路;
use super::scatter_settings::散布の設定;
use super::validation_error::資源検証エラー;

/// チャンク構造とは、1チャンクの道路・建物一覧・散布の設定をまとめた、
/// `チャンク/{x}/{z}/構造`のJSON1本ぶんの内容のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct チャンク構造 {
    pub 道路: チャンクの道路,
    pub 建物一覧: Vec<建物の配置>,
    pub 散布: 散布の設定,
}

impl チャンク構造 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        self.道路.検証する()?;
        for 建物 in &self.建物一覧 {
            建物.検証する()?;
        }
        self.散布.検証する()?;
        Ok(())
    }
}
