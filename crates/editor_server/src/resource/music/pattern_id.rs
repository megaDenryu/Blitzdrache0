//! パターンIDの値オブジェクト。パターン1つを指す安定した名乗りであり、空の文字列を持てない。
//!
//! 曲構成がパターンを配列の位置でなくこの名乗りで指すため、パターンを1つ消しても他の節の指す先が
//! ずれない(参照: `_doc/設計/楽曲エディター.md`「判断8」)。

use serde::{Deserialize, Deserializer, Serialize};

use super::super::validation_error::資源検証エラー;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct パターンID(String);

impl パターンID {
    pub fn 生成する(文字列: impl Into<String>) -> Result<Self, 資源検証エラー> {
        let 文字列 = 文字列.into();
        if 文字列.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        Ok(Self(文字列))
    }

    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for パターンID {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.文字列())
    }
}

impl<'de> Deserialize<'de> for パターンID {
    fn deserialize<入力元: Deserializer<'de>>(入力元: 入力元) -> Result<Self, 入力元::Error> {
        let 文字列 = String::deserialize(入力元)?;
        Self::生成する(文字列).map_err(serde::de::Error::custom)
    }
}
