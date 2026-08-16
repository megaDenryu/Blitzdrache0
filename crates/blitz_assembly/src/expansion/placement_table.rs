//! 部品ごとの配置表: 展開の結果を、部品の種類ごとに配置の列としてまとめたもの。**そのまま描画対象の単位になる。**
//!
//! 据えた順でなく部品ごとにまとめるのは、描画発行が「1部品 + その部品の個体配置の列」を単位とするためである。
//! 100軒ぶんの壁が1つの配置列に入ることで、発行数が軒数に比例しなくなる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「設計の核心」

use blitz_engine::個体配置;

use super::super::part::部品ID;

/// 1つの部品と、その部品を置くすべての配置。
#[derive(Debug, Clone, PartialEq)]
pub struct 部品ごとの配置 {
    識別子: 部品ID,
    配置一覧: Vec<個体配置>,
}

impl 部品ごとの配置 {
    pub fn 識別子(&self) -> &部品ID {
        &self.識別子
    }

    pub fn 配置一覧(&self) -> &[個体配置] {
        &self.配置一覧
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct 部品ごとの配置表 {
    項目一覧: Vec<部品ごとの配置>,
}

impl 部品ごとの配置表 {
    pub(super) fn 空から始める() -> Self {
        Self { 項目一覧: Vec::new() }
    }

    /// 同じ部品が既にあればその配置列へ足し、無ければ新しい項目を作る。並びは最初に現れた順である。
    pub(super) fn 配置を1件足す(&mut self, 識別子: 部品ID, 配置: 個体配置) {
        if let Some(項目) = self.項目一覧.iter_mut().find(|項目| 項目.識別子 == 識別子) {
            項目.配置一覧.push(配置);
            return;
        }
        self.項目一覧.push(部品ごとの配置 {
            識別子,
            配置一覧: vec![配置],
        });
    }

    pub fn 項目一覧(&self) -> &[部品ごとの配置] {
        &self.項目一覧
    }

    /// 描画対象の数になる値。部品の種類数であり、据えた個体の数ではない。
    pub fn 部品の種類数(&self) -> usize {
        self.項目一覧.len()
    }

    pub fn 配置の総数(&self) -> usize {
        self.項目一覧.iter().map(|項目| 項目.配置一覧.len()).sum()
    }
}
