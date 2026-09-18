//! Rust言語構文のパターン生成モジュール(Issue #137)。
//!
//! ソースコード構文解析において照合対象となる宣言構文(impl, struct, enum, const _)を
//! 意味のあるパターンとして構築する。

use crate::ontology::{オントロジー関数型, オントロジートレイト};

/// ソースコード上のデータ型宣言の種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rust型種別 {
    Struct,
    Enum,
}

impl Rust型種別 {
    pub const fn キーワード(self) -> &'static str {
        match self {
            Self::Struct => "struct",
            Self::Enum => "enum",
        }
    }
}

/// 構文解析器がソーステキストから抽出するための構文パターン生成器。
pub struct 構文パターン;

impl 構文パターン {
    /// トレイト実装文の開始パターン(例: `impl M状態 for `)。
    pub fn トレイト実装宣言(トレイト: オントロジートレイト) -> String {
        format!("impl {} for ", トレイト.名前())
    }

    /// 静的型宣言の接頭辞パターン(例: `const _: M遷移関数<`)。
    pub fn 静的型宣言の接頭辞(関数: オントロジー関数型) -> String {
        format!("const _: {}<", 関数.名前())
    }

    /// 公開・非公開の型宣言開始パターン(例: `pub struct Foo`, `struct Foo`)。
    pub fn 型宣言の開始(種別: Rust型種別, 型名: &str) -> [String; 2] {
        let kw = 種別.キーワード();
        [format!("pub {kw} {型名}"), format!("{kw} {型名}")]
    }

    /// 型定義ブロック検出用の部分一致パターン(例: `struct Foo`, `enum Foo`)。
    pub fn 型定義のシグネチャ(型名: &str) -> [String; 2] {
        [format!("struct {型名}"), format!("enum {型名}")]
    }
}
