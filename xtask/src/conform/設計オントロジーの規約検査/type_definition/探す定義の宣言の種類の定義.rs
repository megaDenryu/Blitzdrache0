//! 探す定義の宣言の種類と、その名前の定義の見出しとして照らす宣言の並び。型の定義の探索が、型の定義とトレイトの定義を同じ探し方で探すために使う。

use super::super::syntax_patterns;

/// 探す定義の宣言の種類。型は `struct` と `enum`、トレイトは `trait` の宣言を探す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 探す定義の宣言の種類 {
    型,
    トレイト,
}

impl 探す定義の宣言の種類 {
    /// その名前の定義の見出しとして照らす宣言の並び(`struct 名前`・`enum 名前` か `trait 名前`)。
    pub fn シグネチャ一覧(self, 名前: &str) -> Vec<String> {
        match self {
            Self::型 => syntax_patterns::型定義のシグネチャ(名前).to_vec(),
            Self::トレイト => vec![format!("trait {名前}")],
        }
    }
}
