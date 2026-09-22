//! 本番で有効な行の選別。行番号を維持して試験専用の項目を空行へ写す。
//! 条件を解釈できないファイルは欠落にし、条件付きの関係を事実として採用しない。

use super::{保証範囲の外の構文, 抽出できなかった理由, 抽出できなかった行};
use crate::conform::source_lexing::コードだけの行一覧;
use std::path::Path;

pub fn 本番の行を選ぶ(パス: &Path, 原文: &str) -> (Vec<String>, Vec<抽出できなかった行>) {
    let mut 行一覧 = コードだけの行一覧(原文);
    let mut 添字 = 0;
    while 添字 < 行一覧.len() {
        let 空白なし: String = 行一覧[添字].chars().filter(|文字| !文字.is_whitespace()).collect();
        if 空白なし.contains("#[cfg") || 空白なし.contains("#![cfg") {
            if 空白なし == "#[cfg(not(test))]" || 空白なし == "#![cfg(not(test))]" {
                行一覧[添字].clear();
            } else if 空白なし == "#![cfg(test)]" {
                行一覧.iter_mut().for_each(String::clear);
                break;
            } else if 空白なし == "#[cfg(test)]" {
                let 開始 = 添字;
                let Some(終端) = 試験項目の終端(&行一覧, 添字 + 1) else {
                    return 条件を読めない(パス, 行一覧, 添字);
                };
                添字 = 終端;
                行一覧[開始..添字].iter_mut().for_each(String::clear);
                continue;
            } else {
                return 条件を読めない(パス, 行一覧, 添字);
            }
        }
        添字 += 1;
    }
    (行一覧, Vec::new())
}

fn 条件を読めない(パス: &Path, mut 行一覧: Vec<String>, 添字: usize) -> (Vec<String>, Vec<抽出できなかった行>) {
    let 構文 = 保証範囲の外の構文::本番の条件付きコンパイルを判定できない;
    let 欠落 = 抽出できなかった行::生成する(パス, 添字 + 1, 抽出できなかった理由::保証範囲の外の構文である { 構文 });
    行一覧.iter_mut().for_each(String::clear);
    (行一覧, vec![欠落])
}

// 同じ行の次の本番項目まで捨てないよう、最初の終端より後ろにコードが無いことも確かめる。
fn 試験項目の終端(行一覧: &[String], mut 添字: usize) -> Option<usize> {
    while let Some(行) = 行一覧.get(添字) {
        let 行 = 行.trim();
        if 行.is_empty() || (行.starts_with("#[") && 行.ends_with(']')) {
            添字 += 1;
        } else {
            break;
        }
    }
    let mut 深さ = 0usize;
    for (位置, 行) in 行一覧.iter().enumerate().skip(添字) {
        for (列, 文字) in 行.char_indices() {
            match 文字 {
                '{' | '(' | '[' => 深さ += 1,
                '}' | ')' | ']' => 深さ = 深さ.checked_sub(1)?,
                _ => {}
            }
            if 深さ == 0 && matches!(文字, '}' | ';') {
                return 行[列 + 文字.len_utf8()..].trim().is_empty().then_some(位置 + 1);
            }
        }
    }
    None
}
