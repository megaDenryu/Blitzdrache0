//! 構文検査が照合する宣言の構文パターン(`impl トレイト for `・`struct 型名`・`enum 型名`)。

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

/// 検査が照合する `blitz_design` のトレイト。枝の名前はトレイトの綴りをそのまま写す。
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum オントロジートレイト {
    MDTO,
    Mコマンド,
    Mイベント,
    M規則,
    M状態,
    M入力,
    M設定,
}

impl オントロジートレイト {
    /// `MDTO` を上位トレイトに持つデータの役割。これらを実装する型は `impl MDTO for` も書いていなければならない。
    pub const fn データの役割一覧() -> [Self; 6] {
        [Self::Mコマンド, Self::Mイベント, Self::M規則, Self::M状態, Self::M入力, Self::M設定]
    }

    pub const fn 名前(self) -> &'static str {
        match self {
            Self::MDTO => "MDTO",
            Self::Mコマンド => "Mコマンド",
            Self::Mイベント => "Mイベント",
            Self::M規則 => "M規則",
            Self::M状態 => "M状態",
            Self::M入力 => "M入力",
            Self::M設定 => "M設定",
        }
    }
}

pub struct 構文パターン;

impl 構文パターン {
    /// トレイト実装の行の先頭(例: `impl M状態 for `・`impl blitz_design::M状態 for `)。
    pub fn トレイト実装宣言(トレイト: オントロジートレイト) -> [String; 2] {
        let 名前 = トレイト.名前();
        [format!("impl {名前} for "), format!("impl blitz_design::{名前} for ")]
    }

    /// 型宣言の開始(例: `struct Foo`)。`pub` と `pub(crate)` は含めない。
    pub fn 型宣言の開始(種別: Rust型種別, 型名: &str) -> String {
        format!("{} {型名}", 種別.キーワード())
    }

    /// 型定義のシグネチャ(`struct Foo` と `enum Foo`)。
    pub fn 型定義のシグネチャ(型名: &str) -> [String; 2] {
        [Self::型宣言の開始(Rust型種別::Struct, 型名), Self::型宣言の開始(Rust型種別::Enum, 型名)]
    }
}
