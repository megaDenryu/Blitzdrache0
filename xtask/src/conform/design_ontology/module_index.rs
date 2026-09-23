//! 走査したソースをモジュールパスで引く索引。全ソースから1度だけ組み、モジュールの単位で、在るか・型の定義を持つか・`use` の項目・`as` の別名の元の名前を答え、
//! クレートの単位で、ある名前をどこかの `use … as 名前` が別名として付けているかを答え、ワークスペースの単位で型の別名の表(子のモジュール `module_index/type_alias_table.rs`)を持つ。
//! 実装の対象の型とトレイトの表記を定義へ結び付ける工程(`module_index/name_location_search.rs`・`implementation_binding.rs`・`read_implementation.rs`・`implemented_trait.rs`)が使う。
//! 1度だけ組むのは、自己変更の検査が `M不変データ` の型ごとに全部の実装を読み直し、そのたびにファイルを読み直すと費用が型の数倍に膨らむためである。
//! 索引へ入れる `use` の項目と型の定義と型の別名は、モジュールの直下(`module_path/enclosing_module.rs`。開いている波括弧がすべてモジュールの本体である行)に書いたものだけである。
//! `use` と型の定義は、その行の位置のモジュール(ファイルから推定したモジュールパス + 囲む `mod 名 { … }` の並び)へ振り分けて持つ。Rust の波括弧付きのモジュールは親の `use` を引き継がないため、名前を解く側は実装の位置のモジュールの `use` だけを使う。
//! 関数・実装・トレイトの本体などの局所の位置に書いた `use` と項目は、どのモジュールの取り込みにも宣言にも数えず、ファイルごとの字句位置(`module_index/file_lexical_position.rs`)へ分けて持つ。
//! クレートごとの別名だけは、局所の `use … as` も数える。別名として付けられた名前を広く見るほど、名前を通す判定が違反の側へ倒れるためである。

mod file_lexical_position;
pub mod name_location_search;
mod type_alias_table;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::line_matching::先頭の識別子;
use super::module_path::モジュールパス;
use super::use_resolution::{取り込みの項目, 書き出しの行付きの取り込みの項目一覧};
use file_lexical_position::ファイルの字句位置;
use type_alias_table::型の別名の表;

#[derive(Default)]
pub struct モジュールの索引 {
    モジュール一覧: HashSet<モジュールパス>,                                      // ファイルのモジュールと、モジュールの直下に書いた波括弧付きのモジュール
    モジュールごとの型の名前: HashMap<モジュールパス, HashSet<String>>,           // モジュールの直下の `struct`・`enum` の名前
    モジュールごとの取り込みの項目: HashMap<モジュールパス, Vec<取り込みの項目>>, // 鍵は `use` 文を書いた位置のモジュール
    ファイルごとの字句位置: HashMap<PathBuf, ファイルの字句位置>,
    クレートごとの別名: HashMap<モジュールパス, HashSet<String>>, // クレートごとの、`use … as 名前` が名乗らせた名前の集まり
    型の別名の表: 型の別名の表,
}

impl モジュールの索引 {
    pub fn 全ソースから組む(ソース一覧: &[(PathBuf, Vec<String>)]) -> Self {
        let mut 索引 = Self::default();
        for (パス, 行一覧) in ソース一覧 {
            索引.ファイルを足す(パス, 行一覧);
        }
        索引
    }

    // 1つのファイルの行を、モジュールの直下のものはその位置のモジュールへ、局所の位置のものはそのファイルの字句位置へ振り分ける。
    fn ファイルを足す(&mut self, パス: &Path, 行一覧: &[String]) {
        let モジュール = モジュールパス::ファイルのパスから求める(パス);
        let 行ごとの字句位置 = モジュール.行ごとの字句位置一覧(行一覧);
        self.モジュール一覧.insert(モジュール.clone());
        for (行, 位置) in 行一覧.iter().zip(&行ごとの字句位置).filter(|(_, 位置)| 位置.モジュールの直下か) {
            self.モジュール一覧.insert(位置.モジュール.clone());
            if let Some(型名) = 型の定義の名前(行) {
                self.モジュールごとの型の名前.entry(位置.モジュール.clone()).or_default().insert(型名);
            }
            self.型の別名の表.モジュールの直下の行を足す(行);
        }
        let mut 字句位置 = ファイルの字句位置::局所の行を集めて組む(行ごとの字句位置, 行一覧);
        for (書き出しの行, 項目) in 書き出しの行付きの取り込みの項目一覧(行一覧) {
            self.クレートごとの別名.entry(モジュール.クレート()).or_default().extend(項目.別名.clone());
            match 字句位置.行の字句位置(書き出しの行).filter(|位置| 位置.モジュールの直下か).map(|位置| 位置.モジュール.clone()) {
                Some(位置のモジュール) => self.モジュールごとの取り込みの項目.entry(位置のモジュール).or_default().push(項目),
                None => 字句位置.局所の取り込みを足す(項目),
            }
        }
        self.ファイルごとの字句位置.insert(パス.to_path_buf(), 字句位置);
    }

    /// 走査したファイルの行ごとの字句位置と、局所の位置に書いた `use` と項目が持ち込む名前。走査したファイルでなければ無い。
    pub fn ファイルの字句位置(&self, パス: &Path) -> Option<&ファイルの字句位置> {
        self.ファイルごとの字句位置.get(パス)
    }

    /// そのモジュールが走査範囲にあるか。ファイルのモジュールと、走査したファイルのモジュールの直下の波括弧付きのモジュールが在る。
    pub fn モジュールが在るか(&self, モジュール: &モジュールパス) -> bool {
        self.モジュール一覧.contains(モジュール)
    }

    /// そのモジュールに書かれたパス(`use` の項目のパスかトレイトのパス)の起点のクレート。`crate`・`self`・`super` と、同じクレートの子のモジュールを指す相対のパスは、そのモジュールのクレートである。
    pub fn 書かれたパスのクレート(&self, モジュール: &モジュールパス, パス: &str) -> モジュールパス {
        let 起点 = パス.trim_start_matches("::").split("::").next().unwrap_or_default().trim();
        if matches!(起点, "crate" | "self" | "super") || self.モジュールが在るか(&モジュール.下へ繋ぐ(&[起点])) {
            return モジュール.クレート();
        }
        モジュールパス::区切り一覧から組む(&[起点])
    }

    /// そのモジュールの直下に、その名前の `struct` か `enum` の定義があるか。モジュールが走査範囲に無ければ偽である。
    pub fn 型を定義しているか(&self, モジュール: &モジュールパス, 型名: &str) -> bool {
        self.モジュールごとの型の名前.get(モジュール).is_some_and(|型名一覧| 型名一覧.contains(型名))
    }

    /// そのモジュールの直下に書いた `use` の項目の一覧(波括弧付きのモジュールの中の `use` はそのモジュールの一覧に入り、局所の `use` はどの一覧にも入らない)。無ければ空である。
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

// `struct 名` か `enum 名` の定義の行(前に同じ行の属性と可視性があってもよい)なら、その名前。
fn 型の定義の名前(行: &str) -> Option<String> {
    let 残り = 属性と可視性を読み飛ばす(行);
    let 後ろ = 残り.strip_prefix("struct ").or_else(|| 残り.strip_prefix("enum "))?;
    let 名前 = 先頭の識別子(後ろ.trim_start());
    (!名前.is_empty()).then_some(名前)
}
