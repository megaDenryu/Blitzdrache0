//! ファイルの `use` 行を読む工程。`use` 文を波括弧の群を1段展開した項目の一覧に分け、型名をどのモジュールから取り込んでいるかを求める。
//! 受け取るのはそのファイルのモジュールパスと型名とコードだけの行の一覧、返すのは取り込み元のモジュールパスか、取り込んでいないか、`as` の別名のため取り込み元を求められないかである。
//! `crate::` はクレート名に、`super::` は親に、`self::` は自分のモジュールパスに置き換える。波括弧の群は1段だけ展開し、入れ子の群は展開しない。
//! 頭の属性と可視性(`pub`・`pub(crate)`・`pub(super)`・`pub(self)`・`pub(in パス)`)は、どれも `use` の接頭辞として読み飛ばす。

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::module_path::モジュールパス;

/// `use` 行から取り込み元を求めた結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 取り込み元を求めた結果 {
    取り込んでいない,
    取り込んでいる { モジュールパス: モジュールパス },
    別名のため取り込み元を求められない,
}

/// あるファイルの中で型名がどこから来たかの問い。自分のモジュールパスと型名の組である。
pub struct 取り込み元の問い<'a> {
    pub 自分のモジュールパス: &'a モジュールパス,
    pub 型名: &'a str,
}

impl 取り込み元の問い<'_> {
    pub fn 行一覧から取り込み元を求める(&self, 行一覧: &[String]) -> 取り込み元を求めた結果 {
        let mut 結果 = 取り込み元を求めた結果::取り込んでいない;
        for 項目 in 取り込みの項目一覧(行一覧) {
            if 項目.別名.as_deref() == Some(self.型名) {
                return 取り込み元を求めた結果::別名のため取り込み元を求められない;
            }
            if 項目.別名.is_none() && 項目.元の名前() == self.型名 {
                結果 = 取り込み元を求めた結果::取り込んでいる {
                    モジュールパス: self.自分のモジュールパス.書かれたパスを絶対にする(&項目.パス).親(),
                };
            }
        }
        結果
    }
}

/// `use` 文を波括弧の群を1段展開した1項目。書かれたパス(`crate::a::型`・`a::*`)と、`as` の別名を持つ。
pub struct 取り込みの項目 {
    pub パス: String,
    pub 別名: Option<String>,
}

impl 取り込みの項目 {
    /// パスの最後の区切り(取り込んだ元の名前。glob の取り込みなら `*`)。
    pub fn 元の名前(&self) -> &str {
        self.パス.rsplit("::").next().unwrap_or_default().trim()
    }

    /// この項目がファイルの中で名乗る名前。別名があれば別名、無ければ元の名前である。
    pub fn 名乗る名前(&self) -> &str {
        self.別名.as_deref().unwrap_or_else(|| self.元の名前())
    }

    /// パスの最後の区切りより前(`crate::a::型` なら `crate::a`)。区切りが1つなら空である。
    pub fn 親のパス(&self) -> &str {
        self.パス.rsplit_once("::").map_or("", |(親, _)| 親.trim())
    }

    /// glob の取り込み(`a::*`)か。
    pub fn 全部を取り込むか(&self) -> bool {
        self.元の名前() == "*"
    }
}

/// 行の一覧の `use` 文を、波括弧の群を1段展開した項目の一覧にする。
pub fn 取り込みの項目一覧(行一覧: &[String]) -> Vec<取り込みの項目> {
    use文一覧(行一覧)
        .iter()
        .flat_map(|文| 項目一覧(文))
        .map(|項目| {
            let (パス, 別名) = 項目.split_once(" as ").map_or((項目.as_str(), None), |(パス, 別名)| (パス, Some(別名.trim().to_string())));
            取り込みの項目 { パス: パス.trim().to_string(), 別名 }
        })
        .collect()
}

// `use ` から `;` までを1つの文にする。複数の行にまたがる文は空白で繋ぐ。
fn use文一覧(行一覧: &[String]) -> Vec<String> {
    let mut 文一覧 = Vec::new();
    let mut 途中: Option<String> = None;
    for 行 in 行一覧 {
        let 行 = 行.trim();
        if let Some(続き) = 途中.as_mut() {
            続き.push(' ');
            続き.push_str(行);
        } else if let Some(本文) = 属性と可視性を読み飛ばす(行).strip_prefix("use ") {
            途中 = Some(本文.to_string());
        } else {
            continue;
        }
        if let Some(文) = 途中.take_if(|_| 行.ends_with(';')) {
            文一覧.push(文.trim_end_matches(';').trim().to_string());
        }
    }
    文一覧
}

// `a::{b, c as d}` を `a::b`・`a::c as d` に展開する。波括弧が無ければ文そのものである。
fn 項目一覧(文: &str) -> Vec<String> {
    let Some((接頭辞, 残り)) = 文.split_once('{') else {
        return vec![文.trim().to_string()];
    };
    let 中身 = 残り.rsplit_once('}').map_or(残り, |(中身, _)| 中身);
    中身.split(',').map(str::trim).filter(|項目| !項目.is_empty()).map(|項目| format!("{}{}", 接頭辞.trim(), 項目)).collect()
}
