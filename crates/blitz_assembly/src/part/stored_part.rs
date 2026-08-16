//! 収蔵部品: カタログに収まった部品1件。識別子と定義の組である。
//!
//! 組を裸のタプルで持たないのは、カタログの走査の署名にタプルが現れると、読み手が第1要素と第2要素の
//! どちらが識別子なのかを型から読み取れないためである。

use super::definition::部品定義;
use super::part_id::部品ID;

#[derive(Debug, Clone, PartialEq)]
pub struct 収蔵部品 {
    識別子: 部品ID,
    定義: 部品定義,
}

impl 収蔵部品 {
    pub fn 生成する(識別子: 部品ID, 定義: 部品定義) -> Self {
        Self { 識別子, 定義 }
    }

    pub fn 識別子(&self) -> &部品ID {
        &self.識別子
    }

    pub fn 定義(&self) -> &部品定義 {
        &self.定義
    }
}
