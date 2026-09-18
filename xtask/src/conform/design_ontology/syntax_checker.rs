//! `crates` 配下の各クレートの `src` の下の全ソースを横断して行う構文検査。
//! 照合はすべてコメントと文字列を落としたコードだけの行に対して行う。型の定義の探索は `type_definition.rs` が行う。

use std::path::PathBuf;

use super::super::violation::違反;
use super::syntax_patterns::{Rust型種別, オントロジートレイト, 構文パターン};
use super::type_definition::{型の定義を探す, 定義ブロックの結果};

/// `impl トレイト for 型名` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct トレイト実装型 {
    pub 型名: String,
    pub パス: PathBuf,
    pub 行番号: usize,
}

pub struct クレート構文検査 {
    ソース一覧: Vec<(PathBuf, Vec<String>)>,
    pub(super) 違反一覧: Vec<違反>,
}

impl クレート構文検査 {
    pub const fn 生成する(ソース一覧: Vec<(PathBuf, Vec<String>)>) -> Self {
        Self { ソース一覧, 違反一覧: Vec::new() }
    }

    pub fn 違反一覧(self) -> Vec<違反> {
        self.違反一覧
    }

    pub fn トレイト実装型一覧(&self, トレイト: オントロジートレイト) -> Vec<トレイト実装型> {
        let パターン一覧 = 構文パターン::トレイト実装宣言(トレイト);
        let mut 型一覧 = Vec::new();
        for (パス, 行一覧) in &self.ソース一覧 {
            for (添字, 行) in 行一覧.iter().enumerate() {
                let Some(残り) = パターン一覧.iter().find_map(|パターン| 行.trim().strip_prefix(パターン.as_str())) else {
                    continue;
                };
                let 型名 = 先頭の識別子(残り);
                if !型名.is_empty() {
                    型一覧.push(トレイト実装型 {
                        型名, パス: パス.clone(), 行番号: 添字 + 1
                    });
                }
            }
        }
        型一覧
    }

    /// 型の定義(直前の属性の行を含む)。実装と同じファイルの定義を優先し、決まらなければその旨を返す。
    pub fn 型の定義ブロック(&self, 型: &トレイト実装型) -> 定義ブロックの結果 {
        型の定義を探す(&self.ソース一覧, 型)
    }

    /// 採った定義が `種別` のキーワードで始まるか。定義が一意に決まらないときは偽である。
    pub fn 型種別を満たしているか(&self, 種別: Rust型種別, 型: &トレイト実装型) -> bool {
        let 開始 = 構文パターン::型宣言の開始(種別, &型.型名);
        match self.型の定義ブロック(型) {
            定義ブロックの結果::見つかった(定義) => 定義.lines().any(|行| 語として現れるか(行, &開始)),
            定義ブロックの結果::見つからない | 定義ブロックの結果::複数ある => false,
        }
    }

    /// 採った定義が参照・生ポインタ・内部可変性を持つときの説明。定義が一意に決まらないときは空である。
    pub fn dto純粋性違反一覧(&self, 型: &トレイト実装型) -> Vec<String> {
        let 定義 = match self.型の定義ブロック(型) {
            定義ブロックの結果::見つかった(定義) => 定義,
            定義ブロックの結果::見つからない | 定義ブロックの結果::複数ある => return Vec::new(),
        };
        let mut 違反 = Vec::new();
        if 定義.contains('&') {
            違反.push("の定義は参照(&)を持てません".to_string());
        }
        if 定義.contains("*const") || 定義.contains("*mut") {
            違反.push("の定義は生ポインタを持てません".to_string());
        }
        if let Some(型) = ["RefCell", "Mutex", "RwLock", "Cell", "Atomic"].iter().find(|型| 定義.contains(*型)) {
            違反.push(format!("の定義は内部可変性 `{型}` を持てません"));
        }
        違反
    }

    /// その型の固有の `impl 型名 {` ブロック(` for ` を含まない impl)に `&mut self` があるか。
    pub fn 可変参照メソッドを含むか(&self, 型名: &str) -> bool {
        self.ソース一覧.iter().any(|(_, 行一覧)| {
            let 固有のimplの開始一覧 = 行一覧.iter().enumerate().filter(|(_, 行)| 固有のimplの宣言か(行, 型名));
            固有のimplの開始一覧.into_iter().any(|(開始, _)| 行一覧[開始..=波括弧が閉じる行(行一覧, 開始)].iter().any(|行| 行.contains("&mut self")))
        })
    }
}

/// 先頭から識別子の文字(英数字・下線・非ASCIIの文字)が続く限りを返す。
fn 先頭の識別子(残り: &str) -> String {
    残り.chars().take_while(|文字| 文字.is_alphanumeric() || *文字 == '_').collect()
}

/// `語` が行の中に、前が行頭・空白・`)` で、後ろが識別子の続きでない形で現れるか。
pub(super) fn 語として現れるか(行: &str, 語: &str) -> bool {
    行.match_indices(語).any(|(位置, _)| {
        let 前 = 行[..位置].chars().next_back().is_none_or(|文字| 文字.is_whitespace() || 文字 == ')');
        let 後 = 先頭の識別子(&行[位置 + 語.len()..]).is_empty();
        前 && 後
    })
}

/// 開始の行から波括弧が閉じる行(終端を含む)を返す。開始の行に開き括弧が無ければ開始の行である。
pub(super) fn 波括弧が閉じる行(行一覧: &[String], 開始: usize) -> usize {
    let mut 深さ = 0usize;
    for (添字, 行) in 行一覧.iter().enumerate().skip(開始) {
        深さ += 行.matches('{').count();
        深さ = 深さ.saturating_sub(行.matches('}').count());
        if 深さ == 0 {
            return 添字;
        }
    }
    行一覧.len().saturating_sub(1)
}

/// `impl 型名 {`・`impl<T> 型名<T> {` の行か。` for ` を含む行はトレイト実装であり、含めない。
fn 固有のimplの宣言か(行: &str, 型名: &str) -> bool {
    let 残り = 行.trim_start().strip_prefix("impl").unwrap_or_default();
    let 残り = if 残り.starts_with('<') { 残り.find('>').map_or("", |位置| &残り[位置 + 1..]) } else { 残り };
    !行.contains(" for ") && 先頭の識別子(残り.trim_start()) == 型名
}
