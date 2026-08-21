//! 建物定義IDの値オブジェクト。建物外形カタログの定義1件を指す名前であり、空の綴りを持てない。
//! JSONは裸の文字列として持つが、読み取りは生成の検査を必ず通す(直列化の透過と不変条件の維持を両立させる)。

use serde::{Deserialize, Deserializer, Serialize};

use super::validation_error::資源検証エラー;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct 建物定義ID(String);

impl 建物定義ID {
    pub fn 生成する(綴り: impl Into<String>) -> Result<Self, 資源検証エラー> {
        let 綴り = 綴り.into();
        if 綴り.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        Ok(Self(綴り))
    }

    pub fn 綴り(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for 建物定義ID {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.綴り())
    }
}

impl<'de> Deserialize<'de> for 建物定義ID {
    fn deserialize<入力元: Deserializer<'de>>(入力元: 入力元) -> Result<Self, 入力元::Error> {
        let 綴り = String::deserialize(入力元)?;
        Self::生成する(綴り).map_err(serde::de::Error::custom)
    }
}
