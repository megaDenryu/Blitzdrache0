//! 構文検査が照合する宣言の構文パターン(`impl トレイト for `・`struct 型名`・`enum 型名`)。パターンを組む関数は依存を持たない純粋な関数であり、このモジュールが名前空間を与える。

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

/// 検査が照合する `blitz_design` のトレイト。選択肢の名前はトレイトの名前をそのまま写す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum オントロジートレイト {
    M不変データ,
    Mコマンド,
    Mイベント,
    M規則,
    M状態,
    M入力,
    M設定,
    M値オブジェクト,
    Mエンティティ,
    Mエンティティ識別子,
    M不変エンティティ,
    M可変エンティティ,
}

impl オントロジートレイト {
    /// `M不変データ` を上位トレイトに持つデータの役割。これらを実装する型の定義にも純粋データ規約を当てる(`impl M不変データ for` の置き場所は問わない)。
    pub const fn データの役割一覧() -> [Self; 6] {
        [Self::Mコマンド, Self::Mイベント, Self::M規則, Self::M状態, Self::M入力, Self::M設定]
    }

    /// 存在の分類のマーカー。純粋データ規約と `&mut self` の禁止の対象にはしない(`Mエンティティ` は `M不変データ` を要求せず、`M可変エンティティ` は自己変更を意図する分類であるため)。
    /// `M不変データ` を上位トレイトに持つものは `impl M不変データ for` の行を通して検査される。
    pub const fn 存在の分類一覧() -> [Self; 5] {
        [Self::M値オブジェクト, Self::Mエンティティ, Self::Mエンティティ識別子, Self::M不変エンティティ, Self::M可変エンティティ]
    }

    /// 実装の正規形と再公開の禁止の対象になる全部のマーカー。
    pub fn 全部の一覧() -> Vec<Self> {
        [Self::M不変データ].into_iter().chain(Self::データの役割一覧()).chain(Self::存在の分類一覧()).collect()
    }

    pub const fn 名前(self) -> &'static str {
        match self {
            Self::M不変データ => "M不変データ",
            Self::Mコマンド => "Mコマンド",
            Self::Mイベント => "Mイベント",
            Self::M規則 => "M規則",
            Self::M状態 => "M状態",
            Self::M入力 => "M入力",
            Self::M設定 => "M設定",
            Self::M値オブジェクト => "M値オブジェクト",
            Self::Mエンティティ => "Mエンティティ",
            Self::Mエンティティ識別子 => "Mエンティティ識別子",
            Self::M不変エンティティ => "M不変エンティティ",
            Self::M可変エンティティ => "M可変エンティティ",
        }
    }
}

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
    [型宣言の開始(Rust型種別::Struct, 型名), 型宣言の開始(Rust型種別::Enum, 型名)]
}
