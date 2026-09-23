//! `クレート構文検査` の説明関数のうち、設計解釈マーカーの実装の形を正規形へ固定するもの。触るのは `ソース一覧` と `違反一覧` だけであり、型の定義は探さない。
//! 構文検査は `impl マーカー名 for 型` と `impl blitz_design::マーカー名 for 型` の行だけをマーカーの実装として認識する。それ以外の形(別名の取り込み・再公開の経由・
//! `::blitz_design::` の絶対パス・`blitz_design :: M不変データ` のようなパスの中の空白)で書くと認識できない実装が書け、純粋データ規約などの法則を黙って迂回できるため、
//! マーカーの名前を含む正規形でない実装の行と、別名の取り込みと、`blitz_design` の外での再公開を違反にする。同じ理由で、ファイルの中の `mod 名 { ... }` の
//! 中のマーカーの実装も違反にする(マーカーの実装の置き場をファイルのモジュールの直下へ固定するため。`mod` の中の実装はその `mod` の直下の定義しか指さず、ファイルの直下の同名の型と取り違える)。

use std::path::{Path, PathBuf};

use super::super::violation::違反;
use super::line_matching::{クレート名, トレイト実装の行のトレイトの位置, 波括弧が閉じる行, 語として現れるか};
use super::syntax_checker::クレート構文検査;
use super::syntax_patterns::{self, オントロジートレイト};

const USE_の接頭辞一覧: [&str; 4] = ["use ", "pub use ", "pub(crate) use ", "pub(super) use "];
const 公開USE_の接頭辞一覧: [&str; 3] = ["pub use ", "pub(crate) use ", "pub(super) use "];
const MOD_の接頭辞一覧: [&str; 4] = ["mod ", "pub mod ", "pub(crate) mod ", "pub(super) mod "];

impl クレート構文検査 {
    /// `blitz_design` を `use ... as` で別名にして取り込んでいないこと。クレート・モジュール・トレイトのどの別名も対象であり、複数の行にまたがる `use` も文の終わりまで見る。
    pub fn 設計解釈マーカーを別名で取り込んでいないこと(self) -> Self {
        self.行単位の違反を足す(
            |_, 行一覧| blitz_designの別名の行一覧(行一覧),
            "設計オントロジー: 設計解釈マーカーは `impl M不変データ for 型` または `impl blitz_design::M不変データ for 型` の形だけで実装する。別名で取り込むと conform が実装を認識できない",
        )
    }

    /// 設計解釈マーカーの名前を含むトレイトの実装の行が、`impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` のどちらかであること。
    pub fn 設計解釈マーカーの実装が正規形であること(self) -> Self {
        self.行単位の違反を足す(
            |_, 行一覧| 正規形でないマーカー実装の行一覧(行一覧),
            "設計オントロジー: 設計解釈マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形だけで書く。再公開・絶対パス・パスの中の空白のどれも、構文検査が実装として認識できない",
        )
    }

    /// `blitz_design` の外のファイルが `blitz_design` を公開の `use` で再公開していないこと。`blitz_design` 自身の `lib.rs` が自分のモジュールを再公開するのは対象でない。
    pub fn 設計解釈マーカーを再公開していないこと(self) -> Self {
        self.行単位の違反を足す(
            |パス, 行一覧| if blitz_design自身のファイルか(パス) { Vec::new() } else { blitz_designの再公開の行一覧(行一覧) },
            "設計オントロジー: 設計解釈マーカーを再公開すると、実装の行が正規形でなくなり構文検査が実装を認識できない。`blitz_design` から直接取り込む",
        )
    }

    /// 設計解釈マーカーの実装がファイルの中の `mod 名 { ... }` の中に無いこと。宣言だけの `mod 名;` は対象でない。
    pub fn 設計解釈マーカーの実装が波括弧付きのモジュールの中に無いこと(self) -> Self {
        self.行単位の違反を足す(
            |_, 行一覧| 波括弧付きのモジュールの中のマーカー実装の行一覧(行一覧),
            "設計オントロジー: 設計解釈マーカーの実装をファイルの中の `mod` の中へ置かない(マーカーの実装はファイルのモジュールの直下に置く。`mod` の中の実装はその `mod` の直下の定義しか指さず、ファイルの直下の同名の型と取り違える)",
        )
    }

    // 各ファイルから `行を選ぶ` が返した行番号(1始まり)の全部を、同じ説明の行単位の違反として足す。渡すのは純粋な選択の関数である。
    fn 行単位の違反を足す(mut self, 行を選ぶ: fn(&Path, &[String]) -> Vec<usize>, 説明: &str) -> Self {
        let 該当する行一覧: Vec<(PathBuf, usize)> = self.ソース一覧.iter().flat_map(|(パス, 行一覧)| 行を選ぶ(パス, 行一覧).into_iter().map(|行番号| (パス.clone(), 行番号))).collect();
        for (パス, 行番号) in 該当する行一覧 {
            self.違反一覧.push(違反::行単位(パス, 行番号, 説明.to_string()));
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

// トレイトを書く位置に設計解釈マーカーの名前が識別子として現れるのに、その位置が `マーカー名` でも `blitz_design::マーカー名` でもない行の番号(1始まり)。`::` の周りの空白は正規形でない。
fn 正規形でないマーカー実装の行一覧(行一覧: &[String]) -> Vec<usize> {
    let 名前一覧: Vec<&str> = オントロジートレイト::全部の一覧().into_iter().map(オントロジートレイト::名前).collect();
    let 正規形か = |位置: &str| 名前一覧.iter().any(|名前| 位置 == *名前 || 位置.strip_prefix("blitz_design::") == Some(*名前));
    let マーカーを含むか = |位置: &str| {
        let 区切りを空白にした位置 = 位置.replace("::", " ");
        名前一覧.iter().any(|名前| 語として現れるか(&区切りを空白にした位置, 名前))
    };
    行一覧
        .iter()
        .enumerate()
        .filter_map(|(添字, 行)| トレイト実装の行のトレイトの位置(行).map(|位置| (添字, 位置)))
        .filter(|(_, 位置)| マーカーを含むか(位置) && !正規形か(位置))
        .map(|(添字, _)| 添字 + 1)
        .collect()
}

fn blitz_design自身のファイルか(パス: &Path) -> bool {
    クレート名(パス) == "blitz_design"
}

// 公開の `use` で始まり `blitz_design` を含む行の番号(1始まり)。
fn blitz_designの再公開の行一覧(行一覧: &[String]) -> Vec<usize> {
    行一覧
        .iter()
        .enumerate()
        .filter(|(_, 行)| {
            let 行 = 行.trim();
            公開USE_の接頭辞一覧.iter().any(|接頭辞| 行.starts_with(接頭辞)) && 行.contains("blitz_design")
        })
        .map(|(添字, _)| 添字 + 1)
        .collect()
}

// ファイルの中の `mod 名 {` のブロックの範囲にある、設計解釈マーカーの実装の行の番号(1始まり)。
fn 波括弧付きのモジュールの中のマーカー実装の行一覧(行一覧: &[String]) -> Vec<usize> {
    let パターン一覧: Vec<String> = オントロジートレイト::全部の一覧().into_iter().flat_map(syntax_patterns::トレイト実装宣言).collect();
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
