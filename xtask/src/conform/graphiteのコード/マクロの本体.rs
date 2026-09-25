//! Graphiteのマクロの本体の行とは、原文の中のGraphiteのマクロの呼び出しについて、開きの区切り記号の行と閉じの区切り記号の行に挟まれた行の集まりのことである。
//! 受け取るのは原文、返すのはある行がその集まりに入るかを答える値である。
//!
//! 行数の検査は、この集まりの行を数えから除き、残りの行に1ファイル100行の原則を当てる(2026-09-26のオーナー裁定。Issue #187)。
//! 裁定が100行の制約を当面当てないとしたのは、利用者が手で書くGraphiteのコード(schemaの宣言・`graph!` の中身)だけである。
//! ファイル全体を上限から外すと、マクロを1回呼ぶだけで同じファイルの手書きのRustが何行でも書けるため、外すのはマクロの本体の行に限る。
//! 開きと閉じの区切り記号の行は、呼び出しを書くRustの行として数える。
//!
//! Graphiteのマクロとは、`dynamic_graph_schema!`・`static_graph_schema!`・`graph!` と、静的グラフのinstanceのマクロである。
//! instanceのマクロは名前が利用者の語彙であるため、本体に `generated = "..."` の項を持つ最も内側のマクロの呼び出しとして見分ける。
//! 呼び出しの範囲はコメントと文字列を落としたコードの行の区切り記号の対応で決める。コメントや文字列の中の括弧で範囲がずれないためである。

use std::ops::RangeInclusive;

use super::super::source_lexing::コードだけの行一覧;
use super::手書きの宣言::生成先の項の行番号一覧;

/// 名前で見分けるGraphiteのマクロ。`!` の直前の識別子と比べる。
const 名前で見分けるマクロ一覧: [&str; 3] = ["dynamic_graph_schema", "static_graph_schema", "graph"];

pub struct Graphiteのマクロの本体の行 {
    範囲一覧: Vec<RangeInclusive<usize>>, // 1始まりの行番号の閉区間
}

/// マクロの呼び出し1つが開きの区切り記号から閉じの区切り記号までに占める行。
struct マクロの呼び出し {
    名前: String,
    開きの行: usize,
    閉じの行: usize,
}

impl Graphiteのマクロの本体の行 {
    pub fn 原文から読む(原文: &str) -> Self {
        let 呼び出し一覧 = マクロの呼び出しを集める(原文);
        let mut 本体の呼び出し: Vec<&マクロの呼び出し> = 呼び出し一覧.iter().filter(|呼び出し| 名前で見分けるマクロ一覧.contains(&呼び出し.名前.as_str())).collect();
        for 行番号 in 生成先の項の行番号一覧(原文) {
            let 最も内側 = 呼び出し一覧
                .iter()
                .filter(|呼び出し| 呼び出し.開きの行 <= 行番号 && 行番号 <= 呼び出し.閉じの行)
                .max_by_key(|呼び出し| (呼び出し.開きの行, std::cmp::Reverse(呼び出し.閉じの行)));
            本体の呼び出し.extend(最も内側);
        }
        let 範囲一覧 = 本体の呼び出し.into_iter().map(|呼び出し| 呼び出し.開きの行 + 1..=呼び出し.閉じの行.saturating_sub(1)).filter(|範囲| !範囲.is_empty()).collect();
        Self { 範囲一覧 }
    }

    /// その行(1始まり)が、Graphiteのマクロの本体の行か。
    pub fn 含むか(&self, 行番号: usize) -> bool {
        self.範囲一覧.iter().any(|範囲| 範囲.contains(&行番号))
    }
}

/// コードの行を1文字ずつ辿り、`識別子!` の直後に開く区切り記号から、それに対応して閉じる区切り記号までをマクロの呼び出しとして集める。
fn マクロの呼び出しを集める(原文: &str) -> Vec<マクロの呼び出し> {
    let mut 呼び出し一覧 = Vec::new();
    let mut 開き一覧: Vec<(usize, Option<String>)> = Vec::new();
    let mut 語 = String::new();
    let mut 前の文字 = '\n';
    let mut 直前の非空白 = '\n';
    let mut 感嘆符の前の語: Option<String> = None;
    for (添字, 行) in コードだけの行一覧(原文).iter().enumerate() {
        for 文字 in 行.chars().chain(std::iter::once('\n')) {
            match 文字 {
                _ if 識別子の文字か(文字) => {
                    if !識別子の文字か(前の文字) {
                        語.clear();
                    }
                    語.push(文字);
                }
                '!' => 感嘆符の前の語 = 識別子の文字か(前の文字).then(|| 語.clone()),
                '{' | '(' | '[' => 開き一覧.push((添字 + 1, if 直前の非空白 == '!' { 感嘆符の前の語.take() } else { None })),
                '}' | ')' | ']' => {
                    if let Some((開きの行, Some(名前))) = 開き一覧.pop() {
                        呼び出し一覧.push(マクロの呼び出し {
                            名前, 開きの行, 閉じの行: 添字 + 1
                        });
                    }
                }
                _ => {}
            }
            前の文字 = 文字;
            if !文字.is_whitespace() {
                直前の非空白 = 文字;
            }
        }
    }
    呼び出し一覧
}

fn 識別子の文字か(文字: char) -> bool {
    文字.is_alphanumeric() || 文字 == '_'
}

#[cfg(test)]
#[path = "マクロの本体/マクロの本体の試験.rs"]
mod マクロの本体の試験;
