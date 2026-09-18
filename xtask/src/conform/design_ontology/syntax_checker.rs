//! `crates` 配下の各クレートの `src` の下の全ソースを横断して行う構文検査。
//! 照合はすべてコメントと文字列を落としたコードだけの行に対して行う。行の中の綴りの照合は `line_matching.rs`、型の定義の探索は `type_definition.rs` が行う。

use std::path::PathBuf;

use super::super::violation::違反;
use super::line_matching::{クレート名, 先頭の識別子, 固有のimplの宣言か, 波括弧が閉じる行, 語として現れるか};
use super::syntax_patterns::{Rust型種別, オントロジートレイト, 構文パターン};
use super::type_definition::{型の定義を探す, 定義ブロックの結果};

/// `impl トレイト for 型名` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct トレイト実装型 {
    pub 型名: String,
    pub パス: PathBuf,
    pub 行番号: usize,
}

pub struct クレート構文検査 {
    pub(super) ソース一覧: Vec<(PathBuf, Vec<String>)>,
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
            定義ブロックの結果::見つかった { 定義, .. } => 定義.lines().any(|行| 語として現れるか(行, &開始)),
            定義ブロックの結果::見つからない | 定義ブロックの結果::複数ある => false,
        }
    }

    /// 採った定義が参照・生ポインタ・内部可変性を持つときの説明。定義が一意に決まらないときは空である。
    pub fn dto純粋性違反一覧(&self, 型: &トレイト実装型) -> Vec<String> {
        let 定義 = match self.型の定義ブロック(型) {
            定義ブロックの結果::見つかった { 定義, .. } => 定義,
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

    /// その型の固有の `impl 型名 {` ブロック(` for ` を含まない impl)に `&mut self` があるか。見るのは採った定義と同じクレートのファイルだけであり、
    /// 別のクレートの同名の型の固有の `impl` は数えない。定義が一意に決まらないときは偽である(定義の違反は別に出る)。
    pub fn 可変参照メソッドを含むか(&self, 型: &トレイト実装型) -> bool {
        let 定義ブロックの結果::見つかった { パス: 定義のパス, .. } = self.型の定義ブロック(型) else {
            return false;
        };
        self.ソース一覧.iter().filter(|(パス, _)| クレート名(パス) == クレート名(&定義のパス)).any(|(_, 行一覧)| {
            let 固有のimplの開始一覧 = 行一覧.iter().enumerate().filter(|(_, 行)| 固有のimplの宣言か(行, &型.型名));
            固有のimplの開始一覧.into_iter().any(|(開始, _)| 行一覧[開始..=波括弧が閉じる行(行一覧, 開始)].iter().any(|行| 行.contains("&mut self")))
        })
    }
}
