//! ファイルの `use` 行を読む工程。`use` 文を波括弧の群を入れ子ごと展開した項目の一覧に分ける。受け取るのはコードだけの行の一覧、返すのは読めた項目の一覧と読み切れない文の一覧である。
//! 群の展開そのものは `use_group_expansion.rs` が持つ。書かれたパスを絶対のモジュールパスにするのは、項目を読む側(`module_path.rs`)である。
//! 項目には、その `use` 文を書き出した行の添字を添える(モジュールの索引が項目を位置のモジュールへ振り分けるためである)。
//! 頭の属性と可視性(`pub`・`pub(crate)`・`pub(super)`・`pub(self)`・`pub(in パス)`)は、どれも `use` の接頭辞として読み飛ばす。
//! 読み切れない文(`;` で終わらない文と、展開しきれない群)は黙って飛ばさず、読み切れない宣言として返す。読み口の全域性の検査(`readable_form_assertion.rs`)がそれを違反にする。

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::declaration_reading_outcome::{読めない宣言, 読めない宣言の在り処};
use super::use_group_expansion::群を展開した項目一覧;

/// `use` 文を展開した1項目。書かれたパス(`crate::a::型`・`a::*`)と、`as` の別名を持つ。
pub struct 取り込みの項目 {
    pub パス: String,
    pub 別名: Option<String>,
}

/// 1つのファイルの中の取り込みの項目1件の在り処。書き出しの行はその `use` 文を書き出した行の0始まりの添字である。
pub struct 取り込みの項目の在り処 {
    pub 書き出しの行: usize,
    pub 項目: 取り込みの項目,
}

/// 1つのファイルの `use` 行を読んだ結果。読めた項目の一覧と、読み切れなかった文の一覧を持つ。
#[derive(Default)]
pub struct 読んだ取り込み一覧 {
    pub 項目一覧: Vec<取り込みの項目の在り処>,
    pub 読めない一覧: Vec<読めない宣言の在り処>,
}

impl 取り込みの項目 {
    /// 展開した1項目の表記(`a::b as c`)を、パスと別名へ分けて読む。
    fn 表記から読む(表記: &str) -> Self {
        match 表記.split_once(" as ") {
            Some((パス, 別名)) => Self {
                パス: パス.trim().to_string(),
                別名: Some(別名.trim().to_string()),
            },
            None => Self {
                パス: 表記.trim().to_string(), 別名: None
            },
        }
    }

    /// パスの最後の区切り(取り込んだ元の名前。glob の取り込みなら `*`)。
    pub fn 元の名前(&self) -> &str {
        self.パス.rsplit("::").next().unwrap_or_default().trim()
    }
}

/// 行の一覧の `use` 文を読み、読めた項目と読み切れなかった文に分ける。
pub fn 書き出しの行付きの取り込みの項目一覧(行一覧: &[String]) -> 読んだ取り込み一覧 {
    let mut 読んだ = 読んだ取り込み一覧::default();
    for 文 in 取り込みの文一覧(行一覧) {
        match 群を展開した項目一覧(&文.本文) {
            Ok(表記一覧) => 読んだ.項目一覧.extend(表記一覧.iter().map(|表記| 取り込みの項目の在り処 {
                書き出しの行: 文.書き出しの行,
                項目: 取り込みの項目::表記から読む(表記),
            })),
            Err(理由) => 読んだ.読めない一覧.push(文.読み切れない宣言にする(理由)),
        }
    }
    読んだ
}

// `use ` から `;` までを1つにまとめた文。本文は `use ` と末尾の `;` を除いたものであり、書き出しの綴りは書き出した行そのものである。
struct 取り込みの文 {
    書き出しの行: usize,
    書き出しの綴り: String,
    本文: String,
    終端があるか: bool,
}

impl 取り込みの文 {
    // 終端の無い文の綴りに書き出しの行を使うのは、`;` が現れないまま最後の行まで繋いだ本文が、そのファイルの残り全部になるためである。
    fn 読み切れない宣言にする(&self, 理由: &'static str) -> 読めない宣言の在り処 {
        読めない宣言の在り処 {
            行番号: self.書き出しの行 + 1,
            宣言: 読めない宣言 {
                綴り: if self.終端があるか { format!("use {};", self.本文) } else { self.書き出しの綴り.clone() },
                理由: if self.終端があるか { 理由 } else { "`use` の文が同じファイルの中で `;` で終わっていない" },
            },
        }
    }
}

// `use ` から `;` までを1つの文にし、書き出した行の添字を添える。複数の行にまたがる文は空白で繋ぐ。`;` が最後まで現れない文も、終端の無い文として返す。
fn 取り込みの文一覧(行一覧: &[String]) -> Vec<取り込みの文> {
    let mut 文一覧 = Vec::new();
    let mut 途中: Option<取り込みの文> = None;
    for (添字, 行) in 行一覧.iter().enumerate() {
        let 行 = 行.trim();
        if 途中.is_none() {
            let Some(本文) = 属性と可視性を読み飛ばす(行).strip_prefix("use ") else {
                continue;
            };
            途中 = Some(取り込みの文 {
                書き出しの行: 添字,
                書き出しの綴り: 行.to_string(),
                本文: 本文.to_string(),
                終端があるか: false,
            });
        } else if let Some(文) = 途中.as_mut() {
            文.本文.push(' ');
            文.本文.push_str(行);
        }
        if let Some(mut 文) = 途中.take_if(|_| 行.ends_with(';')) {
            文.本文 = 文.本文.trim_end_matches(';').trim().to_string();
            文.終端があるか = true;
            文一覧.push(文);
        }
    }
    文一覧.extend(途中);
    文一覧
}
