//! 植生定義IDの値オブジェクト。植生定義の正本1件を指す名前であり、空の綴りを持てない。
//! JSONは裸の文字列として持つが、読み取りは生成の検査を必ず通す。エディターチャンクソースを書く側と
//! 読む側の両方がこの型で植生定義を指す。建物定義IDと同じ様式である
//! (参照: `runtime_compilation/building_outline_catalog/building_definition_id.rs`)。

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct 植生定義ID(String);

impl 植生定義ID {
    pub fn 生成する(文字列: impl Into<String>) -> Result<Self, String> {
        let 文字列 = 文字列.into();
        if 文字列.trim().is_empty() {
            return Err("植生定義IDが空である".to_string());
        }
        Ok(Self(文字列))
    }

    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for 植生定義ID {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.文字列())
    }
}

impl<'de> Deserialize<'de> for 植生定義ID {
    fn deserialize<入力元: Deserializer<'de>>(入力元: 入力元) -> Result<Self, 入力元::Error> {
        let 文字列 = String::deserialize(入力元)?;
        Self::生成する(文字列).map_err(serde::de::Error::custom)
    }
}
