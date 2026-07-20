//! 安定ID: アセットをファイルパスでなく論理名で参照するための識別子。
//! パス直接参照を禁じ、カタログ経由の解決を強制する。
//! 参照: `_doc/計画/ユビキタス言語.md`「安定ID（アセットID）」。

use std::fmt;

/// アセットの論理名。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct アセットID(String);

impl アセットID {
    /// 空文字を拒否した安定IDを生成する。
    pub fn 生成する(名前: &str) -> Result<Self, アセットIDエラー> {
        if 名前.is_empty() {
            return Err(アセットIDエラー::空文字);
        }
        Ok(Self(名前.to_string()))
    }

    /// 論理名を文字列として返す。
    pub fn 文字列を返す(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for アセットID {
    fn fmt(&self, フォーマッタ: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(フォーマッタ, "{}", self.0)
    }
}

/// アセットIDの生成失敗。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum アセットIDエラー {
    #[error("アセットIDは空文字にできない")]
    空文字,
}
