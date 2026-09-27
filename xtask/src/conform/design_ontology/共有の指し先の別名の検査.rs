//! `クレート構文検査` の説明関数のうち、共有の指し先(`Arc`・`Rc`・`Weak`)を別名にすることを、純粋データ規約の対象の型を持つクレートで違反にするもの。
//! 共有の指し先の検査(`共有の指し先の検査.rs`)は型の定義の行から `Arc`・`Rc`・`Weak` の名前で表記を探すため、`use std::rc::Rc as 共有;` や
//! `type 共有の地図 = Rc<地図>;` を通すと、`Rc` を持つ定義を名前が見えないまま見逃す。書き手が短い名前を付けるつもりで別名を置くだけで検査を外れるため、
//! 検査が読めない形を黙って通さず、別名そのものを違反にする(グローバル CLAUDE.md「検査器は解析できなかった入力を黙って対象から外してはならない」)。
//! 対象は、純粋データ規約の対象の型を1つでも持つクレートの全ファイルである。別名は定義と別のファイルに置けるため、定義のファイルだけでは足りない。
//! 純粋データ規約の対象の型を持たないクレートが置いた別名を、対象のクレートが取り込んで使う形は保証範囲の外である(`design_ontology.rs` の冒頭が書く)。

use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::violation::違反;
use super::identifier_boundary::識別子として現れる位置一覧;
use super::line_matching::クレート名;
use super::syntax_checker::クレート構文検査;

/// 別名にしてはならない共有の指し先の名前。
const 共有の指し先の名前一覧: [&str; 3] = ["Arc", "Rc", "Weak"];
const 取り込みの接頭辞一覧: [&str; 4] = ["use ", "pub use ", "pub(crate) use ", "pub(super) use "];
const 型の別名の接頭辞一覧: [&str; 4] = ["type ", "pub type ", "pub(crate) type ", "pub(super) type "];

const 別名の説明: &str =
    "設計オントロジー: 純粋データ規約の対象の型を持つクレートでは、共有の指し先(`Arc`・`Rc`・`Weak`)を `use … as` や `type` の別名にしない。共有の指し先の検査は `Arc`・`Rc`・`Weak` の名前で表記を探すため、別名を通した指し先を見逃す";

/// 行を読んでいる途中の文の種類。文は `;` で終わる行まで続く。
#[derive(Clone, Copy)]
enum 読んでいる文 {
    無し,
    取り込み,
    型の別名,
}

impl クレート構文検査 {
    /// 純粋データ規約の対象の型を持つクレートのどのファイルも、`Arc`・`Rc`・`Weak` を `use … as` で別名にせず、右辺に持つ `type` の別名を置いていないこと。
    pub fn 共有の指し先を別名にしていないこと(mut self) -> Self {
        let 対象のクレート一覧: HashSet<OsString> = self.純粋データ規約の対象一覧().iter().map(|型| クレート名(&型.パス).to_os_string()).collect();
        let 該当一覧: Vec<(PathBuf, usize)> = self
            .ソース一覧
            .iter()
            .filter(|(パス, _)| 対象のクレート一覧.contains(クレート名(パス)))
            .flat_map(|(パス, 行一覧)| 共有の指し先の別名の行一覧(行一覧).into_iter().map(|行番号| (パス.clone(), 行番号)))
            .collect();
        for (パス, 行番号) in 該当一覧 {
            self.違反一覧.push(違反::行単位(パス, 行番号, 別名の説明.to_string()));
        }
        self
    }
}

// 共有の指し先を別名にする行の番号(1始まり)。`use` の文では名前の直後に `as` が続く行を、`type` の文では名前が識別子として現れる行を返す。
// 複数の行にまたがる文(`use std::sync::{\n Arc as 共有,\n};`)は `;` で終わる行まで1つの文として見る。
fn 共有の指し先の別名の行一覧(行一覧: &[String]) -> Vec<usize> {
    let mut 該当 = Vec::new();
    let mut 文 = 読んでいる文::無し;
    for (添字, 行) in 行一覧.iter().enumerate() {
        let 行 = 行.trim();
        if matches!(文, 読んでいる文::無し) {
            文 = 文の始まり(行);
        }
        let 別名か = match 文 {
            読んでいる文::無し => false,
            読んでいる文::取り込み => 共有の指し先の名前一覧.iter().any(|名前| 別名にする現れがあるか(行, 名前)),
            読んでいる文::型の別名 => 共有の指し先の名前一覧.iter().any(|名前| !識別子として現れる位置一覧(行, 名前).is_empty()),
        };
        if 別名か {
            該当.push(添字 + 1);
        }
        if 行.ends_with(';') {
            文 = 読んでいる文::無し;
        }
    }
    該当
}

fn 文の始まり(行: &str) -> 読んでいる文 {
    if 取り込みの接頭辞一覧.iter().any(|接頭辞| 行.starts_with(接頭辞)) {
        読んでいる文::取り込み
    } else if 型の別名の接頭辞一覧.iter().any(|接頭辞| 行.starts_with(接頭辞)) {
        読んでいる文::型の別名
    } else {
        読んでいる文::無し
    }
}

// 名前が識別子として現れ、その直後(空白を除く)に `as` と空白が続くか。
fn 別名にする現れがあるか(行: &str, 名前: &str) -> bool {
    識別子として現れる位置一覧(行, 名前)
        .into_iter()
        .any(|位置| 行[位置 + 名前.len()..].trim_start().strip_prefix("as").is_some_and(|後ろ| 後ろ.starts_with(char::is_whitespace)))
}
