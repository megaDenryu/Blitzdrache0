//! 楽曲IDの値オブジェクト。楽曲1件の置き場のファイル名になる名乗りであり、空の綴りと
//! ファイル名として使えない文字を持てない。JSONは裸の文字列として持つが、読み取りは生成の検査を必ず通す。
//!
//! ファイルの中の名乗りと置き場のファイル名が食い違うものは保管庫の読みが拒む
//! (参照: `_doc/設計/楽曲エディター.md`「判断6」)。

use serde::{Deserialize, Deserializer, Serialize};

use super::super::validation_error::資源検証エラー;

const ファイル名として使えない文字一覧: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct 楽曲ID(String);

impl 楽曲ID {
    pub fn 生成する(綴り: impl Into<String>) -> Result<Self, 資源検証エラー> {
        let 綴り = 綴り.into();
        if 綴り.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        if let Some(文字) = 綴り.chars().find(|文字| ファイル名として使えない文字か(*文字)) {
            return Err(資源検証エラー::識別子にファイル名として使えない文字がある { 値: 綴り, 文字 });
        }
        Ok(Self(綴り))
    }

    pub fn 綴り(&self) -> &str {
        &self.0
    }
}

/// 制御文字を併せて拒むのは、置き場のファイル名として書けても、後から一覧に並べたときに人が読めないためである。
fn ファイル名として使えない文字か(文字: char) -> bool {
    文字.is_control() || ファイル名として使えない文字一覧.contains(&文字)
}

impl std::fmt::Display for 楽曲ID {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.綴り())
    }
}

impl<'de> Deserialize<'de> for 楽曲ID {
    fn deserialize<入力元: Deserializer<'de>>(入力元: 入力元) -> Result<Self, 入力元::Error> {
        let 綴り = String::deserialize(入力元)?;
        Self::生成する(綴り).map_err(serde::de::Error::custom)
    }
}
