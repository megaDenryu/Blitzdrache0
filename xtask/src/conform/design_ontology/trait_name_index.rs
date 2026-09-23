//! 走査範囲のトレイトの宣言の名前の集まり。全ソースから1度だけ組み、ある名前のトレイトの宣言が走査範囲にあるかを答える。
//! `use … as` の別名がこの名前を名乗らないこと(`name_uniqueness_assertion.rs`)の検査が引く。
//! 索引へ入れる宣言の位置を限らないのは、名前で引く検査が、関数の本体の中の宣言もマクロの本体の中の宣言も同じ名前として数えるためである。位置で絞ると、絞った位置の宣言が検査から漏れる。

use std::collections::HashSet;
use std::path::PathBuf;

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::line_matching::先頭の識別子;

#[derive(Default)]
pub struct トレイトの名前の索引 {
    名前一覧: HashSet<String>,
}

impl トレイトの名前の索引 {
    pub fn 全ソースから組む(ソース一覧: &[(PathBuf, Vec<String>)]) -> Self {
        let mut 索引 = Self::default();
        for (_, 行一覧) in ソース一覧 {
            索引.名前一覧.extend(行一覧.iter().filter_map(|行| トレイトの宣言の名前(行)));
        }
        索引
    }

    /// 走査範囲にその名前のトレイトの宣言があるか。
    pub fn 宣言しているか(&self, 名前: &str) -> bool {
        self.名前一覧.contains(名前)
    }
}

/// トレイトの宣言の見出しの行なら、そのトレイトの名前。同じ行の属性と可視性(`pub`・`pub(...)`)と `unsafe` を読み飛ばす。
pub fn トレイトの宣言の名前(行: &str) -> Option<String> {
    let 残り = 属性と可視性を読み飛ばす(行);
    let 残り = 残り.strip_prefix("unsafe").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)).map_or(残り, str::trim_start);
    let 後ろ = 残り.strip_prefix("trait")?;
    if !後ろ.starts_with(char::is_whitespace) {
        return None;
    }
    let 名前 = 先頭の識別子(後ろ.trim_start());
    (!名前.is_empty()).then_some(名前)
}
