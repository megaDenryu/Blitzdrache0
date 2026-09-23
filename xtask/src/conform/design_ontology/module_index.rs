//! 走査したソースをモジュールパスで引く索引。全ソースから1度だけ組み、モジュールの単位で、在るか・型の定義を持つか・`use` の項目・`as` の別名の元の名前を答え、
//! クレートの単位で、ある名前をどこかの `use … as 名前` が別名として付けているかを答え、ワークスペースの単位で型の別名の表(子のモジュール `module_index/type_alias_table.rs`)を持つ。
//! 実装の対象の型とトレイトの表記を定義へ結び付ける工程(`module_index/name_location_search.rs`・`implementation_binding.rs`・`read_implementation.rs`・`implemented_trait.rs`)が使う。
//! `use` の項目と型の別名を組むときに1度だけ読むのは、自己変更の検査が `M不変データ` の型ごとに全部の実装を読み直し、そのたびにファイルを読み直すと費用が型の数倍に膨らむためである。
//! `use` の項目は、その文を書いた位置のモジュール(ファイルから推定したモジュールパス + 囲む `mod 名 { … }` の並び。`module_path/enclosing_module.rs`)へ振り分けて持つ。
//! Rust の波括弧付きのモジュールは親の `use` を引き継がないため、名前を解く側は実装の位置のモジュールの `use` だけを使う。波括弧付きのモジュールも在るモジュールに数え、行ごとの位置のモジュールを答える。
//! 型の定義を持つかは、ファイルの単位で答える(波括弧付きのモジュールの中の定義もファイルのモジュールに数える)。

pub mod name_location_search;
mod type_alias_table;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::line_matching::語として現れるか;
use super::module_path::モジュールパス;
use super::syntax_patterns;
use super::use_resolution::{取り込みの項目, 書き出しの行付きの取り込みの項目一覧};
use type_alias_table::型の別名の表;

pub struct モジュールの索引<'a> {
    ソース一覧: &'a [(PathBuf, Vec<String>)],
    モジュールごとのソースの添字: HashMap<モジュールパス, Vec<usize>>,
    モジュールごとの取り込みの項目: HashMap<モジュールパス, Vec<取り込みの項目>>, // 鍵は `use` 文を書いた位置のモジュール
    波括弧付きのモジュールを持つファイルの行ごとの位置: HashMap<PathBuf, Vec<モジュールパス>>,
    波括弧付きのモジュール一覧: HashSet<モジュールパス>,
    クレートごとの別名: HashMap<モジュールパス, HashSet<String>>, // クレートごとの、`use … as 名前` が名乗らせた名前の集まり
    型の別名の表: 型の別名の表,
}

impl<'a> モジュールの索引<'a> {
    pub fn 全ソースから組む(ソース一覧: &'a [(PathBuf, Vec<String>)]) -> Self {
        let mut 索引 = Self {
            ソース一覧,
            モジュールごとのソースの添字: HashMap::new(),
            モジュールごとの取り込みの項目: HashMap::new(),
            波括弧付きのモジュールを持つファイルの行ごとの位置: HashMap::new(),
            波括弧付きのモジュール一覧: HashSet::new(),
            クレートごとの別名: HashMap::new(),
            型の別名の表: 型の別名の表::全ソースから組む(ソース一覧),
        };
        for (添字, (パス, 行一覧)) in ソース一覧.iter().enumerate() {
            let モジュール = モジュールパス::ファイルのパスから求める(パス);
            let 行ごとの位置 = モジュール.行ごとの位置のモジュール一覧(行一覧);
            for (書き出しの行, 項目) in 書き出しの行付きの取り込みの項目一覧(行一覧) {
                索引.クレートごとの別名.entry(モジュール.クレート()).or_default().extend(項目.別名.clone());
                let 位置 = 行ごとの位置.get(書き出しの行).unwrap_or(&モジュール).clone();
                索引.モジュールごとの取り込みの項目.entry(位置).or_default().push(項目);
            }
            let 波括弧付きのモジュール: HashSet<モジュールパス> = 行ごとの位置.iter().filter(|位置| **位置 != モジュール).cloned().collect();
            if !波括弧付きのモジュール.is_empty() {
                索引.波括弧付きのモジュール一覧.extend(波括弧付きのモジュール);
                索引.波括弧付きのモジュールを持つファイルの行ごとの位置.insert(パス.clone(), 行ごとの位置);
            }
            索引.モジュールごとのソースの添字.entry(モジュール).or_default().push(添字);
        }
        索引
    }

    /// そのファイルの1始まりの行番号の行の頭の位置のモジュール。波括弧付きのモジュールを持たないファイルなら、ファイルから推定したモジュールである。
    pub fn 行の位置のモジュール(&self, パス: &Path, 行番号: usize) -> モジュールパス {
        match self.波括弧付きのモジュールを持つファイルの行ごとの位置.get(パス).and_then(|一覧| 一覧.get(行番号.saturating_sub(1))) {
            Some(位置) => 位置.clone(),
            None => モジュールパス::ファイルのパスから求める(パス),
        }
    }

    /// そのモジュールが走査範囲にあるか。ファイルのモジュールと、走査したファイルの中の波括弧付きのモジュールが在る。
    pub fn モジュールが在るか(&self, モジュール: &モジュールパス) -> bool {
        self.モジュールごとのソースの添字.contains_key(モジュール) || self.波括弧付きのモジュール一覧.contains(モジュール)
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

    /// そのモジュールに書いた `use` の項目の一覧(波括弧付きのモジュールの中の `use` は、そのモジュールの一覧に入り、外側の一覧に入らない)。無ければ空である。
    pub fn 取り込みの項目一覧(&self, モジュール: &モジュールパス) -> &[取り込みの項目] {
        self.モジュールごとの取り込みの項目.get(モジュール).map_or(&[][..], Vec::as_slice)
    }

    /// そのモジュールが `use 元 as 別名` で別名を名乗らせているなら、元のパスの最後の名前。
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
