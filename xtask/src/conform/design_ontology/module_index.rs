//! 走査したソースをモジュールパスで引く索引。全ソースから1度だけ組み、モジュールの単位で、在るか・型の定義を持つか・`use` の項目・`as` の別名の元の名前を答え、
//! クレートの単位で、ある名前をどこかの `use … as 名前` が別名として付けているかを答え、ワークスペースの単位で型の別名の表(子のモジュール `module_index/type_alias_table.rs`)を持つ。
//! 実装の対象の型とトレイトの表記を定義へ結び付ける工程(`module_index/name_location_search.rs`・`implementation_binding.rs`・`read_implementation.rs`・`implemented_trait.rs`)が使う。
//! `use` の項目と型の別名を組むときに1度だけ読むのは、自己変更の検査が `M不変データ` の型ごとに全部の実装を読み直し、そのたびにファイルを読み直すと費用が型の数倍に膨らむためである。
//! モジュールパスはファイルのパスから推定した値(`module_path.rs`)であり、同じファイルの中の波括弧付きのモジュール(`mod a { ... }`)は区別しない(`use` の項目は、波括弧付きのモジュールの中に書いたものもファイルのモジュールに数える)。
//! トレイトの宣言の索引(`trait_declaration_index.rs`)は、この索引と別に、宣言を囲む波括弧付きのモジュールを区別する。

pub mod name_location_search;
mod type_alias_table;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::line_matching::語として現れるか;
use super::module_path::モジュールパス;
use super::syntax_patterns;
use super::use_resolution::{取り込みの項目, 取り込みの項目一覧};
use type_alias_table::型の別名の表;

pub struct モジュールの索引<'a> {
    ソース一覧: &'a [(PathBuf, Vec<String>)],
    モジュールごとのソースの添字: HashMap<モジュールパス, Vec<usize>>,
    モジュールごとの取り込みの項目: HashMap<モジュールパス, Vec<取り込みの項目>>,
    クレートごとの別名: HashMap<モジュールパス, HashSet<String>>, // クレートごとの、`use … as 名前` が名乗らせた名前の集まり
    型の別名の表: 型の別名の表,
}

impl<'a> モジュールの索引<'a> {
    pub fn 全ソースから組む(ソース一覧: &'a [(PathBuf, Vec<String>)]) -> Self {
        let mut 索引 = Self {
            ソース一覧,
            モジュールごとのソースの添字: HashMap::new(),
            モジュールごとの取り込みの項目: HashMap::new(),
            クレートごとの別名: HashMap::new(),
            型の別名の表: 型の別名の表::全ソースから組む(ソース一覧),
        };
        for (添字, (パス, 行一覧)) in ソース一覧.iter().enumerate() {
            let モジュール = モジュールパス::ファイルのパスから求める(パス);
            let 項目一覧 = 取り込みの項目一覧(行一覧);
            索引.クレートごとの別名.entry(モジュール.クレート()).or_default().extend(項目一覧.iter().filter_map(|項目| 項目.別名.clone()));
            索引.モジュールごとの取り込みの項目.entry(モジュール.clone()).or_default().extend(項目一覧);
            索引.モジュールごとのソースの添字.entry(モジュール).or_default().push(添字);
        }
        索引
    }

    /// そのモジュールのファイルが走査範囲にあるか。
    pub fn モジュールが在るか(&self, モジュール: &モジュールパス) -> bool {
        self.モジュールごとのソースの添字.contains_key(モジュール)
    }

    /// そのモジュールに書かれたパス(`use` の項目のパスかトレイトのパス)の起点のクレート。`crate`・`self`・`super` と、同じクレートの子のモジュールを指す相対のパスは、そのモジュールのクレートである。
    pub fn 書かれたパスのクレート(&self, モジュール: &モジュールパス, パス: &str) -> モジュールパス {
        let 起点 = パス.trim_start_matches("::").split("::").next().unwrap_or_default().trim();
        if matches!(起点, "crate" | "self" | "super") || self.モジュールが在るか(&モジュール.下へ繋ぐ(&[起点])) {
            return モジュール.クレート();
        }
        モジュールパス::区切り一覧から組む(&[起点])
    }

    /// そのモジュールのファイルに、その名前の `struct` か `enum` の定義があるか。モジュールが走査範囲に無ければ偽である。
    pub fn 型を定義しているか(&self, モジュール: &モジュールパス, 型名: &str) -> bool {
        let 添字一覧 = self.モジュールごとのソースの添字.get(モジュール).map_or(&[][..], Vec::as_slice);
        添字一覧.iter().filter_map(|添字| self.ソース一覧.get(*添字)).any(|(_, 行一覧)| 行一覧が型を定義しているか(行一覧, 型名))
    }

    /// そのモジュールのファイルの `use` の項目の一覧。モジュールが走査範囲に無ければ空である。
    pub fn 取り込みの項目一覧(&self, モジュール: &モジュールパス) -> &[取り込みの項目] {
        self.モジュールごとの取り込みの項目.get(モジュール).map_or(&[][..], Vec::as_slice)
    }

    /// そのモジュールのファイルが `use 元 as 別名` で別名を名乗らせているなら、元のパスの最後の名前。
    pub fn 別名の元の名前(&self, モジュール: &モジュールパス, 別名: &str) -> Option<&str> {
        self.取り込みの項目一覧(モジュール).iter().find(|項目| 項目.別名.as_deref() == Some(別名)).map(取り込みの項目::元の名前)
    }

    /// そのクレートのどこかの `use … as 名前` が、その名前を別名として名乗らせているか。
    pub fn クレートで別名として付けられているか(&self, クレート: &モジュールパス, 名前: &str) -> bool {
        self.クレートごとの別名.get(クレート).is_some_and(|別名一覧| 別名一覧.contains(名前))
    }

    /// 走査範囲の型の別名の表。
    pub fn 型の別名の表(&self) -> &型の別名の表 {
        &self.型の別名の表
    }
}

/// 行の一覧に、その名前の `struct` か `enum` の定義があるか。
pub fn 行一覧が型を定義しているか(行一覧: &[String], 型名: &str) -> bool {
    let シグネチャ = syntax_patterns::型定義のシグネチャ(型名);
    行一覧.iter().any(|行| シグネチャ.iter().any(|宣言| 語として現れるか(行, 宣言)))
}
