//! 型の定義の探索(`type_definition.rs`)が採りうる、同じ名前の `struct`・`enum` の定義1件。定義ブロック(直前の属性の行を含む)と、そのファイルのパスと、定義の行の頭の字句位置を持つ。
//! 字句位置を持つのは、型の同一性の片方である定義のモジュールを、ファイルから推定したモジュールパスでなく、定義の行を囲む `mod 名 { … }` の並びまで繋いだ位置のモジュールで数えるためである。
//! 同じファイルの `mod 内 { pub struct 規則; }` とファイルの直下の `struct 規則;` は別の型であり、関数の本体などの局所の位置の `struct 規則;` はモジュールの型の定義ではない。

use std::path::PathBuf;

use super::super::line_matching::{波括弧が閉じる行, 語として現れるか};
use super::super::module_path::enclosing_module::行の字句位置;
use super::super::module_path::モジュールパス;
use super::super::syntax_patterns;

pub struct 同名の定義の候補 {
    pub 定義: String,
    pub パス: PathBuf,
    pub 位置: 行の字句位置, // 定義の見出しの行の頭の字句位置
}

impl 同名の定義の候補 {
    /// 全ソースから、その名前の `struct`・`enum` の定義を全部集める。同じファイルに2つ以上あればその分だけ並ぶ(黙って最初の1つを採らない)。
    pub fn 全ソースから集める(ソース一覧: &[(PathBuf, Vec<String>)], 型名: &str) -> Vec<Self> {
        let シグネチャ = syntax_patterns::型定義のシグネチャ(型名);
        let mut 候補一覧 = Vec::new();
        for (パス, 行一覧) in ソース一覧 {
            let 開始一覧: Vec<usize> = 行一覧.iter().enumerate().filter(|(_, 行)| シグネチャ.iter().any(|宣言| 語として現れるか(行, 宣言))).map(|(開始, _)| 開始).collect();
            if 開始一覧.is_empty() {
                continue;
            }
            let 字句位置一覧 = モジュールパス::ファイルのパスから求める(パス).行ごとの字句位置一覧(行一覧);
            候補一覧.extend(開始一覧.into_iter().filter_map(|開始| {
                Some(Self {
                    定義: 定義ブロック(行一覧, 開始),
                    パス: パス.clone(),
                    位置: 字句位置一覧.get(開始)?.clone(),
                })
            }));
        }
        候補一覧
    }

    /// その候補が、実装の位置に書いた型名を隠しうる、採らない定義か。実装が局所の位置にあるときの局所の定義と、実装の位置のモジュールのマクロの呼び出しの中の定義がこれである。
    pub fn 実装の名前を隠しうるか(&self, 実装の位置: &行の字句位置) -> bool {
        (self.位置.局所の位置か() && 実装の位置.局所の位置か()) || (self.位置.マクロの呼び出しの中か() && self.位置.モジュール == 実装の位置.モジュール)
    }

    /// その候補が、そのモジュールの直下に書いた定義か。
    pub fn モジュールの直下にあるか(&self, モジュール: &モジュールパス) -> bool {
        self.位置.モジュールの直下か() && self.位置.モジュール == *モジュール
    }
}

// 開始の行の定義を、直前に続く属性の行と空行を含めて1つの文字列にする。
fn 定義ブロック(行一覧: &[String], 開始: usize) -> String {
    let 終了 = 波括弧が閉じる行(行一覧, 開始);
    let 属性開始 = (0..開始).rev().take_while(|添字| 行一覧[*添字].trim().is_empty() || 行一覧[*添字].trim().starts_with("#[")).last().unwrap_or(開始);
    行一覧[属性開始..=終了].join("\n")
}
