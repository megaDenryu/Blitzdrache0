//! 走査で得た観測1件。型の定義とimplブロックという2種の観測を1つの判別共用体で表す。

use super::declaration_amount::宣言の分量;

pub enum 観測 {
    型定義 { 型名: String, 分量: 宣言の分量 },
    実装ブロック { 型名: String, メソッド数: usize },
}

impl 観測 {
    pub fn 型名(&self) -> &str {
        match self {
            Self::型定義 { 型名, .. } | Self::実装ブロック { 型名, .. } => 型名,
        }
    }
}
