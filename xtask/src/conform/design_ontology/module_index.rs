//! 走査したソースをモジュールパスで引く索引。全ソースから1度だけ組み、モジュールの単位で、型の定義を持つか・`use` の項目・`as` の別名の元の名前を答え、
//! クレートの単位で型の別名(`type 名前 = …;`)の元の名前を答える。実装の対象の型とトレイトの表記を定義へ結び付ける工程(`implementation_binding.rs`・`read_implementation.rs`)が使う。
//! `use` の項目と型の別名を組むときに1度だけ読むのは、自己変更の検査が `M不変データ` の型ごとに全部の実装を読み直し、そのたびにファイルを読み直すと費用が型の数倍に膨らむためである。
//! モジュールパスはファイルのパスから推定した値(`module_path.rs`)であり、同じファイルの中の波括弧付きのモジュール(`mod a { ... }`)は区別しない。

use std::collections::HashMap;
use std::path::PathBuf;

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::line_matching::{パスの最後の名前, 先頭の識別子, 語として現れるか};
use super::module_path::モジュールパス;
use super::syntax_patterns;
use super::use_resolution::{取り込みの項目, 取り込みの項目一覧};

pub struct モジュールの索引<'a> {
    ソース一覧: &'a [(PathBuf, Vec<String>)],
    モジュールごとのソースの添字: HashMap<モジュールパス, Vec<usize>>,
    モジュールごとの取り込みの項目: HashMap<モジュールパス, Vec<取り込みの項目>>,
    クレートごとの型の別名: HashMap<モジュールパス, HashMap<String, String>>, // クレートごとの、別名から右辺のパスの最後の名前への対応
}

impl<'a> モジュールの索引<'a> {
    pub fn 全ソースから組む(ソース一覧: &'a [(PathBuf, Vec<String>)]) -> Self {
        let mut 索引 = Self {
            ソース一覧,
            モジュールごとのソースの添字: HashMap::new(),
            モジュールごとの取り込みの項目: HashMap::new(),
            クレートごとの型の別名: HashMap::new(),
        };
        for (添字, (パス, 行一覧)) in ソース一覧.iter().enumerate() {
            let モジュール = モジュールパス::ファイルのパスから求める(パス);
            索引.クレートごとの型の別名.entry(モジュール.クレート()).or_default().extend(行一覧.iter().filter_map(|行| 型の別名の宣言(行)));
            索引.モジュールごとの取り込みの項目.entry(モジュール.clone()).or_default().extend(取り込みの項目一覧(行一覧));
            索引.モジュールごとのソースの添字.entry(モジュール).or_default().push(添字);
        }
        索引
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

    /// そのクレートのどこかに `type 別名 = 右辺;` の宣言があるなら、右辺のパスの最後の名前(型引数を除く)。
    pub fn 型の別名の元の名前(&self, クレート: &モジュールパス, 別名: &str) -> Option<&str> {
        self.クレートごとの型の別名.get(クレート)?.get(別名).map(String::as_str)
    }
}

/// 行の一覧に、その名前の `struct` か `enum` の定義があるか。
pub fn 行一覧が型を定義しているか(行一覧: &[String], 型名: &str) -> bool {
    let シグネチャ = syntax_patterns::型定義のシグネチャ(型名);
    行一覧.iter().any(|行| シグネチャ.iter().any(|宣言| 語として現れるか(行, 宣言)))
}

// `type 別名<..> = 右辺;` の行なら、別名と、右辺のパスの最後の名前の組。
fn 型の別名の宣言(行: &str) -> Option<(String, String)> {
    let 後ろ = 属性と可視性を読み飛ばす(行).strip_prefix("type ")?.trim_start();
    let 右辺 = 後ろ.split_once('=')?.1.split(';').next().unwrap_or_default();
    Some((先頭の識別子(後ろ), パスの最後の名前(右辺.trim()).to_string()))
}
