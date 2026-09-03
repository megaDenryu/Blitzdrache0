//! ファイルの原文から、モジュールの直下(波括弧の深さ0)に書かれた自由関数の署名を集める工程。
//! 受け取るのは原文、返すのは行番号と関数名と引数の並びである。
//!
//! 深さ0だけを見るため、implブロックのメソッド・トレイトの宣言・`mod`の中の関数・入れ子の関数は集めない。
//! 試験の関数は`mod tests`の中にあるため、同じ理由で外れる。マクロが生成する関数は行に現れないため数えない。

use super::parameter::引数へ分ける;
use crate::conform::source_lexing::コードだけの行一覧;
use crate::type_metrics::修飾子を取り除く;

const 署名を読む行数の上限: usize = 40;

pub struct 自由関数の署名 {
    pub 行番号: usize,
    pub 関数名: String,
    pub 引数一覧: Vec<String>,
}

pub fn 自由関数の署名一覧(内容: &str) -> Vec<自由関数の署名> {
    let 行一覧 = コードだけの行一覧(内容);
    let mut 署名一覧 = Vec::new();
    let mut 深さ = 0usize;
    for (添字, 行) in 行一覧.iter().enumerate() {
        if 深さ == 0 && 修飾子を取り除く(行.trim()).starts_with("fn ") {
            署名一覧.extend(署名を読み取る(&行一覧, 添字));
        }
        深さ = 次の深さ(深さ, 行);
    }
    署名一覧
}

fn 次の深さ(現在: usize, 行: &str) -> usize {
    行.chars().fold(現在, |深さ, 文字| match 文字 {
        '{' => 深さ + 1,
        '}' => 深さ.saturating_sub(1),
        _ => 深さ,
    })
}

/// 引数の並びは複数行へ折り返されるため、丸括弧が釣り合うまで行をつなげて1つの綴りにする。
fn 署名を読み取る(行一覧: &[String], 開始添字: usize) -> Option<自由関数の署名> {
    let mut 綴り = String::new();
    for 行 in 行一覧.iter().skip(開始添字).take(署名を読む行数の上限) {
        綴り.push_str(行.trim());
        綴り.push(' ');
        let 残り = 修飾子を取り除く(&綴り).strip_prefix("fn ")?.trim_start();
        let 関数名: String = 残り.chars().take_while(|文字| 文字.is_alphanumeric() || *文字 == '_').collect();
        let Some(引数の綴り) = 残り.get(関数名.len()..).and_then(ジェネリクスを飛ばす).and_then(丸括弧の中身) else {
            continue;
        };
        return Some(自由関数の署名 {
            行番号: 開始添字 + 1,
            関数名,
            引数一覧: 引数へ分ける(&引数の綴り),
        });
    }
    None
}

/// 関数名に続く型引数の並びを読み飛ばす。境界の中の`->`は閉じ括弧と区別できないため、直前の文字で除く。
fn ジェネリクスを飛ばす(綴り: &str) -> Option<&str> {
    let 整形 = 綴り.trim_start();
    if !整形.starts_with('<') {
        return Some(整形);
    }
    let mut 深さ = 0usize;
    let mut 直前 = ' ';
    for (位置, 文字) in 整形.char_indices() {
        match 文字 {
            '<' => 深さ += 1,
            '>' if 直前 != '-' => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return 整形.get(位置 + 1..);
                }
            }
            _ => {}
        }
        直前 = 文字;
    }
    None
}

fn 丸括弧の中身(綴り: &str) -> Option<String> {
    let 開始 = 綴り.find('(')?;
    let mut 深さ = 0usize;
    for (位置, 文字) in 綴り.get(開始..)?.char_indices() {
        match 文字 {
            '(' => 深さ += 1,
            ')' => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return 綴り.get(開始 + 1..開始 + 位置).map(str::to_string);
                }
            }
            _ => {}
        }
    }
    None
}
