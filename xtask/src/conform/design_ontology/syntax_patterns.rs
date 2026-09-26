//! 構文検査が照合する宣言の構文パターン(`impl トレイト for `・`struct 型名`・`enum 型名`)。パターンを組む関数は依存を持たない純粋な関数であり、このモジュールが名前空間を与える。

use super::設計解釈マーカーの一覧::設計解釈マーカー;

/// 型の定義の種別。
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

/// トレイト実装の行の先頭(例: `impl M状態 for `・`impl blitz_design::M状態 for `)。
pub fn トレイト実装宣言(マーカー: 設計解釈マーカー) -> [String; 2] {
    let 名前 = マーカー.名前();
    [format!("impl {名前} for "), format!("impl blitz_design::{名前} for ")]
}

/// 型宣言の開始(例: `struct Foo`)。`pub` と `pub(crate)` は含めない。
pub fn 型宣言の開始(種別: Rust型種別, 型名: &str) -> String {
    format!("{} {型名}", 種別.キーワード())
}

/// 型定義のシグネチャ(`struct Foo` と `enum Foo`)。
pub fn 型定義のシグネチャ(型名: &str) -> [String; 2] {
    [型宣言の開始(Rust型種別::Struct, 型名), 型宣言の開始(Rust型種別::Enum, 型名)]
}
