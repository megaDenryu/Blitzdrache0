//! 部品ID: カタログの中で1つの部品を指す識別子。
//!
//! 裸の文字列で持たないのは、接合点名・ファイル名・材質名といった同じ形の文字列が周りに並んでおり、
//! どれを渡しても型が通るためである。取り違えは組み立ての結果が変わるまで見えない。

use std::fmt;

use super::error::カタログエラー;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 部品ID {
    綴り: String,
}

impl 部品ID {
    /// 空の識別子を拒む。前後の空白は落とす。
    pub fn 生成する(綴り: &str) -> Result<Self, カタログエラー> {
        let 整えた綴り = 綴り.trim();
        if 整えた綴り.is_empty() {
            return Err(カタログエラー::部品IDが空);
        }
        Ok(Self {
            綴り: 整えた綴り.to_string(),
        })
    }

    pub fn 綴り(&self) -> &str {
        &self.綴り
    }
}

impl fmt::Display for 部品ID {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.綴り)
    }
}
