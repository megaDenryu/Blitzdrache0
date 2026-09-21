//! 関数の設計上の役割の語彙。役割ごとに、名前と取る型引数の数と、型引数の位置ごとに出す設計関係の種類を答える。
//!
//! この語彙を使用箇所の読み取りから分けるのは、触る状態が違うためである。こちらは役割そのものの定義(名前と型引数の意味)を所有し、
//! 読み取りの側は1行のRustの構文を所有する。役割を1つ足すときに直すのはこのファイルだけである。
//! 参照: `crates/blitz_design/src/function_role.rs`・`crates/blitz_esca/src/ontology.rs`。

use crate::design_model::設計関係の種類;

/// 関数の設計上の役割。`M射影関数`・`M解釈関数` は `blitz_design::function_role`、`M遷移関数` は `blitz_esca::ontology` に在る。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 関数の役割 {
    M解釈関数,
    M射影関数,
    M遷移関数,
}

impl 関数の役割 {
    /// 3つの役割の全件。
    pub const fn 全部の一覧() -> [Self; 3] {
        [Self::M解釈関数, Self::M射影関数, Self::M遷移関数]
    }

    pub const fn 名前(self) -> &'static str {
        match self {
            Self::M解釈関数 => "M解釈関数",
            Self::M射影関数 => "M射影関数",
            Self::M遷移関数 => "M遷移関数",
        }
    }

    /// その役割が取る型引数の数。
    pub const fn 型引数の数(self) -> usize {
        match self {
            Self::M解釈関数 | Self::M射影関数 => 2,
            Self::M遷移関数 => 5,
        }
    }

    /// 型引数の位置と、その位置の型に対して出す関係の種類。`M遷移関数` の第5の型引数(失敗)は、失敗が生成される事実ではないため関係を出さない。
    pub const fn 型引数の位置ごとの関係(self) -> &'static [(usize, 設計関係の種類)] {
        match self {
            Self::M解釈関数 | Self::M射影関数 => &[(0, 設計関係の種類::消費する), (1, 設計関係の種類::生成する)],
            Self::M遷移関数 => &[(0, 設計関係の種類::消費する), (1, 設計関係の種類::消費する), (2, 設計関係の種類::参照する), (3, 設計関係の種類::生成する)],
        }
    }
}
