//! 構文検査が照合する宣言の構文パターン(`impl トレイト for `・`struct 型名`・`enum 型名`・`const _: 関数型<`)。

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

/// 検査が照合する `blitz_design` のトレイト。`M状態` と `M入力` の規則はコンパイラが強制するため持たない。枝の名前はトレイトの綴りをそのまま写す。
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum オントロジートレイト {
    MDTO,
    Mコマンド,
    Mイベント,
    M規則,
}

impl オントロジートレイト {
    pub const fn 名前(self) -> &'static str {
        match self {
            Self::MDTO => "MDTO",
            Self::Mコマンド => "Mコマンド",
            Self::Mイベント => "Mイベント",
            Self::M規則 => "M規則",
        }
    }
}

/// 検査が照合する関数の役割の関数型。枝の名前は関数型の綴りをそのまま写す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum オントロジー関数型 {
    M射影関数,
    M解釈関数,
    M遷移関数,
}

impl オントロジー関数型 {
    pub const fn 全部() -> [Self; 3] {
        [Self::M射影関数, Self::M解釈関数, Self::M遷移関数]
    }

    pub const fn 名前(self) -> &'static str {
        match self {
            Self::M射影関数 => "M射影関数",
            Self::M解釈関数 => "M解釈関数",
            Self::M遷移関数 => "M遷移関数",
        }
    }

    /// その関数型の型引数の境界をコンパイラに確かめさせる恒等の関数の名前。
    pub const fn 確認関数の名前(self) -> &'static str {
        match self {
            Self::M射影関数 => "射影関数として確認する",
            Self::M解釈関数 => "解釈関数として確認する",
            Self::M遷移関数 => "遷移関数として確認する",
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

    /// 関数役割の静的な宣言の接頭辞(例: `const _: M遷移関数<`)。
    pub fn 静的型宣言の接頭辞(関数: オントロジー関数型) -> String {
        format!("const _: {}<", 関数.名前())
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
