//! Escaオントロジーの構文規則を機械検証するための構文解析器(Issue #137)。
use crate::ontology::{オントロジー関数型, オントロジートレイト};
use crate::syntax_patterns::{構文パターン, Rust型種別};

/// クレート内の全ソースコードを横断して構文検査を行う操作DTO。
#[derive(Clone, Copy)]
pub struct クレート構文検査<'a> {
    pub ソース一覧: &'a [&'a str],
}

impl<'a> クレート構文検査<'a> {
    pub fn トレイト実装型一覧(&self, トレイト: オントロジートレイト) -> Vec<&'a str> {
        let パターン = 構文パターン::トレイト実装宣言(トレイト);
        let mut 型一覧 = Vec::new();
        for ソース in self.ソース一覧 {
            for 行 in ソース.lines() {
                if let Some(残り) = 行.trim().strip_prefix(&パターン) {
                    if let Some(型名) = 残り.split_whitespace().next() {
                        型一覧.push(型名.trim_end_matches('{').trim());
                    }
                }
            }
        }
        型一覧
    }

    pub fn 型の定義ブロック(&self, 型名: &str) -> String {
        let シグネチャ = 構文パターン::型定義のシグネチャ(型名);
        for ソース in self.ソース一覧 {
            let 行一覧: Vec<&str> = ソース.lines().collect();
            let mut 宣言 = None;
            for (i, 行) in 行一覧.iter().enumerate() {
                let 行 = 行.trim();
                if 行.contains(&シグネチャ[0]) || 行.contains(&シグネチャ[1]) {
                    宣言 = Some(i);
                    break;
                }
            }
            if let Some(開始) = 宣言 {
                let 属性開始 = 開始.saturating_sub(2);
                let (mut 深さ, mut 終了) = (0, 開始);
                for (i, 行) in 行一覧.iter().enumerate().skip(開始) {
                    深さ += 行.matches('{').count();
                    深さ -= 行.matches('}').count();
                    if 深さ == 0 && 行.contains('}') {
                        終了 = i;
                        break;
                    }
                }
                return 行一覧[属性開始..=終了].join("\n");
            }
        }
        String::new()
    }

    pub fn 型種別を満たしているか(&self, 種別: Rust型種別, 型名: &str) -> bool {
        let 開始一覧 = 構文パターン::型宣言の開始(種別, 型名);
        self.ソース一覧.iter().any(|s| {
            s.lines().map(str::trim).any(|l| l.starts_with(&開始一覧[0]) || l.starts_with(&開始一覧[1]))
        })
    }

    pub fn 宣言の型引数一覧(&self, 関数: オントロジー関数型) -> Vec<Vec<String>> {
        let 接頭辞 = 構文パターン::静的型宣言の接頭辞(関数);
        let mut 結果 = Vec::new();
        for ソース in self.ソース一覧 {
            for 行 in ソース.lines() {
                if let Some(残り) = 行.trim().strip_prefix(&接頭辞) {
                    if let Some(中身) = 残り.split('>').next() {
                        結果.push(中身.split(',').map(|s| s.trim().to_string()).collect());
                    }
                }
            }
        }
        結果
    }

    pub fn dto純粋性違反一覧(&self, 型名: &str) -> Vec<String> {
        let 定義 = self.型の定義ブロック(型名);
        let mut 違反 = Vec::new();
        if !定義.contains("Clone") { 違反.push(format!("MDTO `{型名}` は Clone を導出していません")); }
        if 定義.contains('&') { 違反.push(format!("MDTO `{型名}` は参照(&)を持てません")); }
        if 定義.contains("*const") || 定義.contains("*mut") { 違反.push(format!("MDTO `{型名}` は生ポインタを持てません")); }
        for 型 in ["RefCell", "Mutex", "RwLock", "Cell", "Atomic"] {
            if 定義.contains(型) { 違反.push(format!("MDTO `{型名}` は内部可変性 `{型}` を持てません")); }
        }
        違反
    }

    pub fn 可変参照メソッドを含むか(&self) -> bool {
        self.ソース一覧.iter().any(|ソース| ソース.contains("&mut self"))
    }
}
