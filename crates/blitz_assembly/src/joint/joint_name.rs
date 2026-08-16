//! 接合点名: 部品の中で1つの接合点を指す名前。同じ部品の中では一意である。
//!
//! 裸の文字列で持たないのは、組み立て規則が接合点を名前で指すためである。空の名前や、部品IDと取り違えた文字列を
//! そのまま受け取ると、指す先が決まらないまま組み立てが進む。

use std::fmt;

use super::error::接合点エラー;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 接合点名 {
    綴り: String,
}

impl 接合点名 {
    /// 空の名前を拒む。前後の空白は落とす。Blenderの生成スクリプトが綴りを組み立てる過程で
    /// 空白が混ざることがあり、それを別名として扱うと一意性の検査が空振りする。
    pub fn 生成する(綴り: &str) -> Result<Self, 接合点エラー> {
        let 整えた綴り = 綴り.trim();
        if 整えた綴り.is_empty() {
            return Err(接合点エラー::名前が空);
        }
        Ok(Self {
            綴り: 整えた綴り.to_string(),
        })
    }

    pub fn 綴り(&self) -> &str {
        &self.綴り
    }
}

impl fmt::Display for 接合点名 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.綴り)
    }
}
