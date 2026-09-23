//! 1つのファイルの `macro_rules!` の本体1つ。名前と本体が占める行の範囲を持ち、本体の中の `impl` の外にある自己変更の関数を答える。
//! 本体の範囲は、`macro_rules!` の後ろで最初に開いた括弧の種類(`{`・`(`・`[`)の対応を数えて求める。開き括弧は名前の次の行にあってもよく、`macro_rules! 名前 ( ... );` と `[ ... ];` の形も読む。
//! 本体の中の関数は、`impl` の中にあるかによらず、生えた先の型を検査器が決められない。`impl` の本体の中の関数はその実装の見出しから読むため(`unbound_implementation.rs`)、ここでは `impl` の本体の外の関数だけを答える。
//! 本体の中の関数で数えるのは、受け手が可変のものと、問う名前(`Self` と、マーカーを名乗る型の名前の閉包)への可変参照を引数に持つものである。
//! 問う名前を `Self` だけにしないのは、マクロの本体へ具体の型名を書いた関数(`fn 足す(対象: &mut 規則)`)が落ちるためである。

use std::ops::RangeInclusive;

use super::declaration_prefix::先頭の属性を読み飛ばす;
use super::function_signature::{自己変更を問う対象, 関数の署名, 関数の署名を読む};
use super::identifier_boundary::識別子の文字か;
use super::impl_header::implの見出しを読む;
use super::line_matching::先頭の識別子;

pub struct マクロの本体 {
    pub 名前: String,
    範囲: RangeInclusive<usize>, // 0始まりの行の添字。`macro_rules!` の行から本体を閉じる括弧の行まで
}

impl マクロの本体 {
    /// ファイルの行一覧から、`macro_rules!` の本体をすべて集める(行頭の属性 `#[macro_export]` を剥がしてから判定する)。
    pub fn ファイルから集める(行一覧: &[String]) -> Vec<Self> {
        (0..行一覧.len())
            .filter_map(|開始| {
                let 後ろ = 先頭の属性を読み飛ばす(&行一覧[開始]).strip_prefix("macro_rules!")?;
                Some(Self {
                    名前: 先頭の識別子(後ろ.trim_start()),
                    範囲: 開始..=本体を閉じる行(行一覧, 開始),
                })
            })
            .collect()
    }

    /// `macro_rules!` を書き出した行の0始まりの添字。
    pub fn 書き出しの行(&self) -> usize {
        *self.範囲.start()
    }

    pub fn 行を含むか(&self, 行: usize) -> bool {
        self.範囲.contains(&行)
    }

    /// 本体の中の `impl` の本体の外にある関数のうち、受け手が可変か、問う名前への可変参照を引数に持つものの一覧。
    pub fn 実装の外の自己変更の関数一覧(&self, 行一覧: &[String], 問う名前一覧: &[String]) -> Vec<マクロの中の自己変更の関数> {
        let 実装の本体一覧: Vec<RangeInclusive<usize>> = self.範囲.clone().filter_map(|開始| implの見出しを読む(行一覧, 開始)).map(|見出し| 見出し.本体の開始行..=見出し.本体の終了行).collect();
        let mut 一覧 = Vec::new();
        for 行 in self.範囲.clone().filter(|行| !実装の本体一覧.iter().any(|本体| 本体.contains(行))) {
            let 残りの本体: String = 行一覧.iter().take(self.範囲.end() + 1).skip(行).map(String::as_str).collect::<Vec<_>>().join("\n");
            let この行の長さ = 行一覧[行].len();
            let 関数一覧 = 語の始まり一覧(&残りの本体, "fn").into_iter().filter(|位置| *位置 < この行の長さ).filter_map(|位置| 関数の署名を読む(&残りの本体[位置..]));
            一覧.extend(関数一覧.filter_map(|署名| 自己変更を与える相手(問う名前一覧, &署名).map(|対象の名前| マクロの中の自己変更の関数 { 行, 署名, 対象の名前 })));
        }
        一覧
    }
}

/// マクロの本体の中の `impl` の外にある自己変更の関数1件。行の0始まりの添字と署名と、自己変更を与える相手の名前を持つ。
pub struct マクロの中の自己変更の関数 {
    pub 行: usize,
    pub 署名: 関数の署名,
    pub 対象の名前: String,
}

// その関数が自己変更を与える相手の名前。問う名前のどれにも与えなければ無い。
fn 自己変更を与える相手(問う名前一覧: &[String], 署名: &関数の署名) -> Option<String> {
    問う名前一覧
        .iter()
        .find(|名前| {
            (自己変更を問う対象 {
                型名: 名前.as_str(),
                可変参照を対象にするか: false,
            })
            .自己変更を与えるか(署名)
        })
        .cloned()
}

// 表記の中で、前が識別子の文字でない位置に現れる語の開始位置の一覧。
fn 語の始まり一覧(表記: &str, 語: &str) -> Vec<usize> {
    表記.match_indices(語).map(|(位置, _)| 位置).filter(|位置| 表記[..*位置].chars().next_back().is_none_or(|前| !識別子の文字か(前))).collect()
}

// `macro_rules!` の行から、その後ろで最初に開いた括弧の種類の対応を数え、本体を閉じる行を求める。閉じなければ最後の行である。
fn 本体を閉じる行(行一覧: &[String], 開始: usize) -> usize {
    let mut 対の括弧: Option<(char, char)> = None;
    let mut 深さ = 0usize;
    for (添字, 行) in 行一覧.iter().enumerate().skip(開始) {
        let 読む部分 = if 添字 == 開始 { 行.split_once("macro_rules!").map_or("", |(_, 後ろ)| 後ろ) } else { 行.as_str() };
        for 文字 in 読む部分.chars() {
            let (開き, 閉じ) = match 対の括弧 {
                Some(対) => 対,
                None => match 文字 {
                    '{' => *対の括弧.insert(('{', '}')),
                    '(' => *対の括弧.insert(('(', ')')),
                    '[' => *対の括弧.insert(('[', ']')),
                    _ => continue,
                },
            };
            if 文字 == 開き {
                深さ += 1;
            } else if 文字 == 閉じ {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return 添字;
                }
            }
        }
    }
    行一覧.len().saturating_sub(1)
}
