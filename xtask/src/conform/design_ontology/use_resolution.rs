//! ファイルの `use` 行が型名をどのモジュールから取り込んでいるかを求める工程。呼び出し連鎖の中の独立した工程であり、受け取るのはそのファイルのモジュールパスと型名と
//! コードだけの行の一覧、返すのは取り込み元のモジュールパスか、取り込んでいないか、`as` の別名のため取り込み元を求められないかである。
//! `crate::` はクレート名に、`super::` は親に、`self::` は自分のモジュールパスに置き換える。波括弧の群は1段だけ展開し、入れ子の群は展開しない。

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
        for 文 in use文一覧(行一覧) {
            for 項目 in 項目一覧(&文) {
                let (パス, 別名) = 項目.split_once(" as ").map_or((項目.as_str(), None), |(パス, 別名)| (パス.trim(), Some(別名.trim())));
                if 別名 == Some(self.型名) {
                    return 取り込み元を求めた結果::別名のため取り込み元を求められない;
                }
                if 別名.is_none() && パス.rsplit("::").next() == Some(self.型名) {
                    結果 = 取り込み元を求めた結果::取り込んでいる {
                        モジュールパス: self.自分のモジュールパス.書かれたパスを絶対にする(パス).親(),
                    };
                }
            }
        }
        結果
    }
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
        } else if let Some(本文) = ["use ", "pub use ", "pub(crate) use ", "pub(super) use "].iter().find_map(|接頭辞| 行.strip_prefix(接頭辞)) {
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
