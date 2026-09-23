//! 1つのファイルの行ごとの字句位置(`module_path/enclosing_module.rs`)と、局所の位置(モジュールの直下でない所。関数・実装・トレイトの本体や `const` のブロックの内側)に書いた `use` と項目が持ち込む名前。
//! モジュールの索引は、局所の `use` と項目をモジュールの取り込みにも宣言にも数えない。Rust ではそれらを書いた波括弧の内側でしか見えないためである。
//! ただし局所の位置に書いた実装(関数の本体の中の `impl`)が書いた名前は、局所の `use` と項目が隠しうる。その実装の名前を解く工程(`read_implementation.rs`)だけが、隠しうるかを問うためにこれを読む。
//! 局所の名前は、どの波括弧の内側かを区別せずファイルの単位で持つ。隠しうる名前を広く見る側へ倒し、隠しうる名前を書いた局所の実装を、黙って外さず違反の側へ倒すためである。

use std::collections::HashSet;

use super::super::declaration_prefix::属性と可視性を読み飛ばす;
use super::super::line_matching::先頭の識別子;
use super::super::module_path::enclosing_module::行の字句位置;
use super::super::use_resolution::取り込みの項目;
use super::type_alias_table::型の別名の宣言;

/// 型と同じ名前空間へ名前を持ち込む項目の予約語。
const 名前を持ち込む項目の予約語一覧: [&str; 6] = ["struct", "enum", "union", "type", "trait", "mod"];

pub struct ファイルの字句位置 {
    行ごとの字句位置: Vec<行の字句位置>,
    局所で宣言した名前一覧: HashSet<String>,       // 局所の `struct`・`enum`・`union`・`type`・`trait`・`mod` の名前
    局所の取り込みの項目一覧: Vec<取り込みの項目>, // 局所の `use` の項目
    局所の型の別名一覧: Vec<(String, String)>,     // 局所の `type 別名 = 右辺;` の別名と右辺の最後の名前の組
}

impl ファイルの字句位置 {
    /// 行ごとの字句位置と行の一覧から、局所の位置の行が持ち込む名前を集めて組む。局所の `use` の項目は呼び出し側が `局所の取り込みを足す` で足す。
    pub fn 局所の行を集めて組む(行ごとの字句位置: Vec<行の字句位置>, 行一覧: &[String]) -> Self {
        let mut 字句位置 = Self {
            行ごとの字句位置,
            局所で宣言した名前一覧: HashSet::new(),
            局所の取り込みの項目一覧: Vec::new(),
            局所の型の別名一覧: Vec::new(),
        };
        for (行, 位置) in 行一覧.iter().zip(&字句位置.行ごとの字句位置) {
            if !位置.モジュールの直下か {
                字句位置.局所で宣言した名前一覧.extend(項目の宣言の名前(行));
                字句位置.局所の型の別名一覧.extend(型の別名の宣言(行));
            }
        }
        字句位置
    }

    pub fn 局所の取り込みを足す(&mut self, 項目: 取り込みの項目) {
        self.局所の取り込みの項目一覧.push(項目);
    }

    /// 0始まりの添字の行の頭の字句位置。
    pub fn 行の字句位置(&self, 添字: usize) -> Option<&行の字句位置> {
        self.行ごとの字句位置.get(添字)
    }

    /// 0始まりの添字の行が局所の位置にあり、局所の項目か `use` がその名前を持ち込みうるか。局所の glob の取り込みがあれば、どの名前も持ち込みうる。
    pub fn 局所の名前が隠しうるか(&self, 添字: usize, 名前: &str) -> bool {
        self.局所の位置か(添字) && (self.局所で宣言した名前一覧.contains(名前) || self.局所の取り込みの項目一覧.iter().any(|項目| 項目.名乗る名前() == 名前 || 項目.全部を取り込むか()))
    }

    /// 0始まりの添字の行が局所の位置にあり、局所の `use 元 as 別名` か `type 別名 = 元;` がその別名で型名を指しうるか。
    pub fn 局所の別名が型名を指しうるか(&self, 添字: usize, 別名: &str, 型名: &str) -> bool {
        self.局所の位置か(添字) && (self.局所の取り込みの項目一覧.iter().any(|項目| 項目.別名.as_deref() == Some(別名) && 項目.元の名前() == 型名) || self.局所の型の別名一覧.iter().any(|(左辺, 右辺)| 左辺 == 別名 && 右辺 == 型名))
    }

    fn 局所の位置か(&self, 添字: usize) -> bool {
        self.行ごとの字句位置.get(添字).is_some_and(|位置| !位置.モジュールの直下か)
    }
}

// 名前を持ち込む項目の宣言の行なら、その名前。同じ行の属性と可視性と `unsafe` を読み飛ばす。
fn 項目の宣言の名前(行: &str) -> Option<String> {
    let 残り = 属性と可視性を読み飛ばす(行);
    let 残り = 残り.strip_prefix("unsafe ").map_or(残り, str::trim_start);
    let 後ろ = 名前を持ち込む項目の予約語一覧.iter().find_map(|予約語| 残り.strip_prefix(予約語).filter(|後ろ| 後ろ.starts_with(char::is_whitespace)))?;
    let 名前 = 先頭の識別子(後ろ.trim_start());
    (!名前.is_empty()).then_some(名前)
}
