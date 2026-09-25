//! 部品ID: カタログの中で1つの部品を指す識別子。
//!
//! 裸の文字列で持たないのは、接合点名・ファイル名・材質名といった同じ形の文字列が周りに並んでおり、
//! どれを渡しても型が通るためである。取り違えは組み立ての結果が変わるまで見えない。

use std::fmt;

use super::error::カタログエラー;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 部品ID {
    文字列: String,
}

impl 部品ID {
    /// 空の識別子を拒む。前後の空白は落とす。
    pub fn 生成する(文字列: &str) -> Result<Self, カタログエラー> {
        let 整えた文字列 = 文字列.trim();
        if 整えた文字列.is_empty() {
            return Err(カタログエラー::部品IDが空);
        }
        Ok(Self { 文字列: 整えた文字列.to_string() })
    }

    pub fn 文字列(&self) -> &str {
        &self.文字列
    }
}

impl fmt::Display for 部品ID {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.文字列)
    }
}
