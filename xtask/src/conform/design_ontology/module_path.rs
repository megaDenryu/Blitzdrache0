//! Rust のモジュールパス(`クレート名::a::b` の形)。型の同一性(定義のモジュールパス + 型名)の片方を担う値であり、ファイルのパスから自分を導き、親と子を答える。
//! `crates` の直下のクレート `名前` について、`src` の直下の `lib.rs`・`main.rs` は `名前`、`src` の下の `a/b.rs` は `名前::a::b`、`a/mod.rs` は `名前::a` である。
//! 試験のファイル(`tests.rs`・`*_tests.rs`)も同じ規則で自分のモジュールになる。
//! この推定は同じファイルの中の波括弧付きのモジュール(`mod a { ... }`)によるファイルとモジュールの不一致を区別できない。そのため `marker_form_assertion.rs` が `mod` の中のマーカーの実装を違反にし、
//! `type_definition.rs` が同じファイルの同名の定義の重複を一意に決まらないとして違反にする。
//! `#[path = "..."] mod` については、`module_structure_assertion.rs` が宣言された論理のモジュールパスとこの推定の一致を確かめるため、この推定はそのままで成り立つ。

pub mod enclosing_module;

use std::path::{Component, Path};

/// Rustのソースのファイルの末尾。モジュールパスの導出がこれを落とし、`module_declaration.rs` の正規形の表記がこれを付ける。
pub const ソースのファイルの末尾: &str = ".rs";

/// モジュールパス。`crates` の下に無いファイルのモジュールパスは空である。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct モジュールパス(String);

impl モジュールパス {
    /// ファイルのパスからそのファイルのモジュールパスを導く。
    pub fn ファイルのパスから求める(パス: &Path) -> Self {
        let mut 部品一覧 = パス.components().filter_map(|部品| match 部品 {
            Component::Normal(名前) => 名前.to_str(),
            _ => None,
        });
        部品一覧.find(|部品| *部品 == "crates");
        let Some(クレート) = 部品一覧.next() else {
            return Self(String::new());
        };
        let mut 区切り一覧 = vec![クレート];
        for 部品 in 部品一覧.skip_while(|部品| *部品 == "src") {
            let 名前 = 部品.strip_suffix(ソースのファイルの末尾).unwrap_or(部品);
            if !matches!(名前, "lib" | "main" | "mod") {
                区切り一覧.push(名前);
            }
        }
        Self::区切り一覧から組む(&区切り一覧)
    }

    /// `::` で区切った区切りの列からモジュールパスを組む。外部クレートのパス(`blitz_design::M不変データ` 等)はこの形で組む。
    pub fn 区切り一覧から組む(区切り一覧: &[&str]) -> Self {
        Self(区切り一覧.join("::"))
    }

    /// `クレート名::a::b` の1行の表記。設計関係グラフの節点の識別子がこの表記を持ち、モジュール構造の一致検査の違反の説明が論理と物理のモジュールパスを並べて書くために使う。
    pub fn 表記(&self) -> &str {
        &self.0
    }

    /// 親のモジュールパス。最上位(クレート)の親はそのクレート自身である。
    pub fn 親(&self) -> Self {
        Self(self.0.rsplit_once("::").map_or(self.0.clone(), |(親, _)| 親.to_string()))
    }

    /// 最上位のクレートのモジュールパス。
    pub fn クレート(&self) -> Self {
        Self(self.0.split("::").next().unwrap_or_default().to_string())
    }

    /// 自分の下へ区切りの列を繋いだモジュールパス。空の列なら自分自身であり、自分が空なら列だけから組む。
    pub fn 下へ繋ぐ(&self, 区切り一覧: &[&str]) -> Self {
        match (self.0.is_empty(), 区切り一覧.is_empty()) {
            (true, _) => Self::区切り一覧から組む(区切り一覧),
            (false, true) => self.clone(),
            (false, false) => Self(format!("{}::{}", self.0, 区切り一覧.join("::"))),
        }
    }

    /// このモジュールに書かれたパスの表記(`crate::a`・`super::a`・`self::a`・外部クレートの `blitz_x::a`)を絶対のモジュールパスにする。
    /// `crate::` はクレート名に、`super::` は親に、`self::` は自分に置き換え、起点の予約語で始まらないパスはそのまま外部クレートのパスとして組む。
    pub fn 書かれたパスを絶対にする(&self, パス: &str) -> Self {
        let mut 区切り一覧 = パス.split("::").map(str::trim).peekable();
        let 起点 = match 区切り一覧.peek().copied() {
            Some("crate") => {
                区切り一覧.next();
                Some(self.クレート())
            }
            Some("self") => {
                区切り一覧.next();
                Some(self.clone())
            }
            Some("super") => {
                let mut 現在 = self.clone();
                while 区切り一覧.peek().copied() == Some("super") {
                    現在 = 現在.親();
                    区切り一覧.next();
                }
                Some(現在)
            }
            _ => None,
        };
        let 残り: Vec<&str> = 区切り一覧.collect();
        match 起点 {
            Some(起点) => 起点.下へ繋ぐ(&残り),
            None => Self::区切り一覧から組む(&残り),
        }
    }

    /// このモジュールに書かれたパスの表記が指しうるモジュールの2つ。起点の予約語か外部クレートからの絶対のパスと、このモジュールの下への相対のパスである。
    /// 相対を足すのは、`lib.rs` の `pub use traveler::X` の `traveler` が同じクレートの子のモジュールを指すためである。
    pub fn 書かれたパスが指しうるモジュール一覧(&self, パス: &str) -> [Self; 2] {
        let 区切り一覧: Vec<&str> = パス.split("::").map(str::trim).collect();
        [self.書かれたパスを絶対にする(パス), self.下へ繋ぐ(&区切り一覧)]
    }
}
