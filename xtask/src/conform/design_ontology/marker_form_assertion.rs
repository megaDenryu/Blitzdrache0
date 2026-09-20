//! `クレート構文検査` の説明関数のうち、設計マーカーの実装の形を正規形へ固定するもの。触るのは `ソース一覧` と `違反一覧` だけであり、型の定義は探さない。
//! 構文検査は `impl マーカー名 for 型` と `impl blitz_design::マーカー名 for 型` の行だけをマーカーの実装として認識する。`blitz_design` を別名で取り込むと
//! 認識できない実装が書け、純粋データ規約などの法則を黙って迂回できるため、別名の取り込みそのものを違反にする。同じ理由で、ファイルの中の `mod 名 { ... }` の
//! 中のマーカーの実装も違反にする(型の定義をファイル単位で探すため、同じファイルの別の `mod` の同名の型と取り違える)。

use std::path::PathBuf;

use super::super::violation::違反;
use super::line_matching::波括弧が閉じる行;
use super::syntax_checker::クレート構文検査;
use super::syntax_patterns::{self, オントロジートレイト};

const USE_の接頭辞一覧: [&str; 4] = ["use ", "pub use ", "pub(crate) use ", "pub(super) use "];
const MOD_の接頭辞一覧: [&str; 4] = ["mod ", "pub mod ", "pub(crate) mod ", "pub(super) mod "];

impl クレート構文検査 {
    /// `blitz_design` を `use ... as` で別名にして取り込んでいないこと。クレート・モジュール・トレイトのどの別名も対象であり、複数の行にまたがる `use` も文の終わりまで見る。
    pub fn 設計マーカーを別名で取り込んでいないこと(mut self) -> Self {
        let 該当する行一覧: Vec<(PathBuf, usize)> = self.ソース一覧.iter().flat_map(|(パス, 行一覧)| blitz_designの別名の行一覧(行一覧).into_iter().map(|行番号| (パス.clone(), 行番号))).collect();
        for (パス, 行番号) in 該当する行一覧 {
            self.違反一覧.push(違反::行単位(
                パス,
                行番号,
                "設計オントロジー: 設計マーカーは `impl MDTO for 型` または `impl blitz_design::MDTO for 型` の形だけで実装する。別名で取り込むと conform が実装を認識できない".to_string(),
            ));
        }
        self
    }

    /// 設計マーカーの実装がファイルの中の `mod 名 { ... }` の中に無いこと。宣言だけの `mod 名;` は対象でない。
    pub fn 設計マーカーの実装が波括弧付きのモジュールの中に無いこと(mut self) -> Self {
        let 該当する行一覧: Vec<(PathBuf, usize)> = self
            .ソース一覧
            .iter()
            .flat_map(|(パス, 行一覧)| 波括弧付きのモジュールの中のマーカー実装の行一覧(行一覧).into_iter().map(|行番号| (パス.clone(), 行番号)))
            .collect();
        for (パス, 行番号) in 該当する行一覧 {
            self.違反一覧.push(違反::行単位(
                パス,
                行番号,
                "設計オントロジー: 設計マーカーの実装をファイルの中の `mod` の中へ置かない(構文検査は型の定義をファイル単位で探すため、同じファイルの別の `mod` の同名の型と取り違える)".to_string(),
            ));
        }
        self
    }
}

// `blitz_design` を含む `use` の文の中で ` as ` を持つ行の番号(1始まり)。複数の行にまたがる文は `;` の行まで1つの文として見る。
fn blitz_designの別名の行一覧(行一覧: &[String]) -> Vec<usize> {
    let mut 該当 = Vec::new();
    let mut 文の中 = false;
    for (添字, 行) in 行一覧.iter().enumerate() {
        let 行 = 行.trim();
        if !文の中 {
            文の中 = USE_の接頭辞一覧.iter().any(|接頭辞| 行.starts_with(接頭辞)) && 行.contains("blitz_design");
        }
        if 文の中 && 行.contains(" as ") {
            該当.push(添字 + 1);
        }
        if 行.ends_with(';') {
            文の中 = false;
        }
    }
    該当
}

// ファイルの中の `mod 名 {` のブロックの範囲にある、設計マーカーの実装の行の番号(1始まり)。
fn 波括弧付きのモジュールの中のマーカー実装の行一覧(行一覧: &[String]) -> Vec<usize> {
    let パターン一覧: Vec<String> = [オントロジートレイト::MDTO].into_iter().chain(オントロジートレイト::データの役割一覧()).flat_map(syntax_patterns::トレイト実装宣言).collect();
    let mut 該当 = Vec::new();
    for (開始, 行) in 行一覧.iter().enumerate() {
        let 行 = 行.trim();
        if !(MOD_の接頭辞一覧.iter().any(|接頭辞| 行.starts_with(接頭辞)) && 行.contains('{')) {
            continue;
        }
        let 終了 = 波括弧が閉じる行(行一覧, 開始);
        該当.extend((開始..=終了).filter(|添字| パターン一覧.iter().any(|パターン| 行一覧[*添字].trim().starts_with(パターン.as_str()))).map(|添字| 添字 + 1));
    }
    該当
}
