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
    M観測,
    M工程,
    M値オブジェクト,
    Mエンティティ,
    Mエンティティ識別子,
    M不変エンティティ,
    M可変エンティティ,
    MParameter,
    MOptions,
    M結果,
}

impl オントロジートレイト {
    /// データの役割の軸(`M不変データ` を除く。`M不変データ` はデータの役割の一種でなく、複数のデータの役割が共有する基底の契約である)。正規形・別名・再公開・波括弧付きのモジュールの検査の対象である。
    pub const fn データの役割一覧() -> [Self; 6] {
        [Self::Mコマンド, Self::Mイベント, Self::M規則, Self::M状態, Self::M入力, Self::M観測]
    }

    /// 処理の役割の軸。`M工程` はデータの役割に含めない。`M不変データ` を上位トレイトに持たないため純粋データ規約と `&mut self` の禁止の対象にせず、正規形・別名・再公開・波括弧付きのモジュールの検査の対象にはする。
    /// 同じ層に属する関数の役割(`M射影関数`・`M解釈関数`・`M遷移関数`)は境界付きの newtype であってトレイトのマーカーではないため、この一覧に現れない。
    pub const fn 処理の役割一覧() -> [Self; 1] {
        [Self::M工程]
    }

    /// `M不変データ` を上位トレイトに持つデータの役割。これらを実装する型の定義にも純粋データ規約を当てる(`impl M不変データ for` の置き場所は問わない)。
    /// 検査器が型の定義をたどる必要があるのはこの一覧と `M不変データ` と `MParameter` と `M結果`(標準の `Result` を除く)だけである。
    pub const fn 純粋データの役割一覧() -> [Self; 6] {
        [Self::Mコマンド, Self::Mイベント, Self::M規則, Self::M状態, Self::M入力, Self::M観測]
    }

    /// 存在の分類の軸。純粋データ規約と `&mut self` の禁止の対象にはしない(`Mエンティティ` は `M不変データ` を要求しないため)。`M不変データ` を上位トレイトに持つものは `impl M不変データ for` の行を通して検査される。
    pub const fn 存在の分類一覧() -> [Self; 3] {
        [Self::M値オブジェクト, Self::Mエンティティ, Self::Mエンティティ識別子]
    }

    /// 変更可能性の軸(エンティティの不変と可変)。同じ型が両方を名乗ることは `排他の分類を同時に名乗っていないこと` が違反にする。
    pub const fn 変更可能性一覧() -> [Self; 2] {
        [Self::M不変エンティティ, Self::M可変エンティティ]
    }

    /// 関数境界の役割の軸。`MParameter` と `MOptions` は `M不変データ` を上位トレイトに持つため、純粋データ規約と `&mut self` の禁止は `impl M不変データ for` の行を通して掛かる。
    /// `MParameter` には加えて `Option<` のフィールドを持たないことを課す(`parameter_assertion.rs`)。同じ型が両方を名乗ることは `排他の分類を同時に名乗っていないこと` が違反にする。
    /// `M結果` は上位トレイトを持たず、純粋データ規約と `&mut self` の禁止を課さない(`純粋データの役割一覧` に入れない)。結果は判別型であるという構文の法則として、実装の対象の型が `enum` であることだけを課す(`すべての結果が列挙型であること`)。
    pub const fn 関数境界の役割一覧() -> [Self; 3] {
        [Self::MParameter, Self::MOptions, Self::M結果]
    }

    /// 実装の正規形と再公開の禁止の対象になる全部のマーカー。5つの軸を連ねる。
    pub fn 全部の一覧() -> Vec<Self> {
        [Self::M不変データ]
            .into_iter()
            .chain(Self::データの役割一覧())
            .chain(Self::処理の役割一覧())
            .chain(Self::存在の分類一覧())
            .chain(Self::変更可能性一覧())
            .chain(Self::関数境界の役割一覧())
            .collect()
    }

    /// 同じ軸の排他の分類の組。同じ型が組の両方を名乗ることは意味として同時に成り立たない。
    pub const fn 排他の分類の組一覧() -> [(Self, Self); 2] {
        [(Self::M不変エンティティ, Self::M可変エンティティ), (Self::MParameter, Self::MOptions)]
    }

    pub const fn 名前(self) -> &'static str {
        match self {
            Self::M不変データ => "M不変データ",
            Self::Mコマンド => "Mコマンド",
            Self::Mイベント => "Mイベント",
            Self::M規則 => "M規則",
            Self::M状態 => "M状態",
            Self::M入力 => "M入力",
            Self::M観測 => "M観測",
            Self::M工程 => "M工程",
            Self::M値オブジェクト => "M値オブジェクト",
            Self::Mエンティティ => "Mエンティティ",
            Self::Mエンティティ識別子 => "Mエンティティ識別子",
            Self::M不変エンティティ => "M不変エンティティ",
            Self::M可変エンティティ => "M可変エンティティ",
            Self::MParameter => "MParameter",
            Self::MOptions => "MOptions",
            Self::M結果 => "M結果",
        }
    }
}

/// `M結果` の実装の対象が標準の `Result` か。`Result` は標準ライブラリの型であり `crates` 配下に定義を持たないため、定義をたどらない特例である。
/// 見るのは対象の型名(`Result<T, E>` の先頭の識別子)だけであり、`std::result::Result` のようにパスで修飾した書き方と、`Result` という名前の独自の型は区別しない。
pub fn 標準のresultへの実装か(対象の型名: &str) -> bool {
    対象の型名 == "Result"
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
