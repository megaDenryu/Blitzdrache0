//! 木の規則が指す部品と、その部品のどの接合点を使うかの指定。
//!
//! 部品IDと接合点名の組を役割ごとの型にするのは、どちらも同じ形の値であり、並べて渡すと取り違えても型が通るためである。
//! 幹の頂部へ葉房の根元を継ぐ誤りは、絵にして初めて見える。

use crate::joint::接合点名;

use super::super::part::部品ID;

/// 幹の一節。上下に継ぐ2つの面と、枝を生やす側面の1つを持つ。
#[derive(Debug, Clone, PartialEq)]
pub struct 幹の指定 {
    pub 部品: 部品ID,
    pub 基部: 接合点名,
    pub 頂部: 接合点名,
    pub 枝の生え口: 接合点名,
}

/// 幹の側面から生える枝。根元で幹へ付き、頂芽で葉房を受ける。
#[derive(Debug, Clone, PartialEq)]
pub struct 枝の指定 {
    pub 部品: 部品ID,
    pub 根元: 接合点名,
    pub 頂芽: 接合点名,
}

/// 枝の先に付く葉の房。根元だけを持ち、そこから先へは何も継がない。
#[derive(Debug, Clone, PartialEq)]
pub struct 葉房の指定 {
    pub 部品: 部品ID,
    pub 根元: 接合点名,
}
