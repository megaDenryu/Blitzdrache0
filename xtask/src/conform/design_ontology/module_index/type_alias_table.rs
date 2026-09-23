//! 走査範囲の型の別名(`type 名前 = 右辺;`)の表。全ソースから1度だけ組み、別名が型名を指しうるかを答える。実装の対象の型の表記を定義へ結び付ける工程(`implementation_binding.rs`)が使う。
//! 表はワークスペース全体で1つにする。別のクレートの `pub type 別名 = 規則;` を取り込んで `impl 変更 for 別名` と書く実装も、型名 `規則` を指しうるためである。
//! 同じ名前の別名が複数あれば右辺を全部持つ。1つだけ残すと、後に読んだ別名が先の別名を上書きし、先の別名を通した実装を黙って外すためである。
//! 別名として拾うのは、モジュールの直下(`mod 名前 { ... }` の中を含む)の `type` だけである。実装とトレイトの本体の中の関連型(`type Output = Self;`)は型の別名ではない。

use std::collections::HashMap;
use std::path::PathBuf;

use super::super::declaration_prefix::属性と可視性を読み飛ばす;
use super::super::line_matching::{パスの最後の名前, 先頭の識別子};

pub struct 型の別名の表 {
    別名ごとの右辺の名前: HashMap<String, Vec<String>>, // 別名から、右辺のパスの最後の名前(型引数を除く)の一覧への対応
}

impl 型の別名の表 {
    pub fn 全ソースから組む(ソース一覧: &[(PathBuf, Vec<String>)]) -> Self {
        let mut 別名ごとの右辺の名前: HashMap<String, Vec<String>> = HashMap::new();
        for (_, 行一覧) in ソース一覧 {
            for (別名, 右辺の名前) in モジュールの直下の型の別名一覧(行一覧) {
                別名ごとの右辺の名前.entry(別名).or_default().push(右辺の名前);
            }
        }
        Self { 別名ごとの右辺の名前 }
    }

    /// 走査範囲のどこかの `type 別名 = 右辺;` の右辺の最後の名前が型名か。
    pub fn 別名が型名を指しうるか(&self, 別名: &str, 型名: &str) -> bool {
        self.別名ごとの右辺の名前.get(別名).is_some_and(|右辺一覧| 右辺一覧.iter().any(|右辺| 右辺 == 型名))
    }
}

// 1つのファイルの、モジュールの直下の `type` の別名と右辺の最後の名前の組の一覧。開いている波括弧がすべて `mod` の本体である行だけを読む。
fn モジュールの直下の型の別名一覧(行一覧: &[String]) -> Vec<(String, String)> {
    let mut 開いた波括弧はモジュールか: Vec<bool> = Vec::new();
    let mut 一覧 = Vec::new();
    for 行 in 行一覧 {
        if 開いた波括弧はモジュールか.iter().all(|モジュールか| *モジュールか) {
            一覧.extend(型の別名の宣言(行));
        }
        let モジュールの見出しか = 属性と可視性を読み飛ばす(行).starts_with("mod ");
        for 文字 in 行.chars() {
            match 文字 {
                '{' => 開いた波括弧はモジュールか.push(モジュールの見出しか),
                '}' => {
                    開いた波括弧はモジュールか.pop();
                }
                _ => {}
            }
        }
    }
    一覧
}

// `type 別名<..> = 右辺;` の行なら、別名と、右辺のパスの最後の名前の組。
fn 型の別名の宣言(行: &str) -> Option<(String, String)> {
    let 後ろ = 属性と可視性を読み飛ばす(行).strip_prefix("type ")?.trim_start();
    let 右辺 = 後ろ.split_once('=')?.1.split(';').next().unwrap_or_default();
    Some((先頭の識別子(後ろ), パスの最後の名前(右辺.trim()).to_string()))
}
