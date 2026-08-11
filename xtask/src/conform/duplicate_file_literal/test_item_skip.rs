//! `#[cfg(test)]`の付いた項目が占める行の範囲を求める工程。受け取るのは原文、返すのは除外する行の範囲一覧である。
//!
//! 属性が付くのは次の1項目だけであり、ファイルの末尾までではない。`#[cfg(test)] mod tests;`のように
//! 1行で閉じる宣言もあれば、`#[cfg(test)] mod tests { ... }`のように波括弧で閉じる項目もある。
//! ファイルの途中で打ち切ると、その下に並ぶ本番のリテラルが検査から丸ごと落ちる。
//!
//! 数えるのはコードだけの行である。コメントの中の`#[cfg(test)]`や、文字列の中の波括弧を数えないためである。

use crate::conform::source_lexing::コードだけの行一覧;

const 試験の属性: &str = "#[cfg(test)]";

pub(super) fn 試験の項目の行範囲一覧(内容: &str) -> Vec<(usize, usize)> {
    let コード行一覧 = コードだけの行一覧(内容);
    let mut 範囲一覧: Vec<(usize, usize)> = Vec::new();
    for (添字, 行) in コード行一覧.iter().enumerate() {
        let 行番号 = 添字 + 1;
        if !行.contains(試験の属性) || 範囲一覧.last().is_some_and(|(_, 終わり)| 行番号 <= *終わり) {
            continue;
        }
        範囲一覧.push((行番号, 項目の終わりの行を求める(&コード行一覧, 添字)));
    }
    範囲一覧
}

pub(super) fn 範囲の中の行か(範囲一覧: &[(usize, usize)], 行番号: usize) -> bool {
    範囲一覧.iter().any(|(始まり, 終わり)| (*始まり..=*終わり).contains(&行番号))
}

/// 属性の行から項目の終わりを探す。波括弧が1度でも開いたらその対応が閉じた行で終わり、
/// 開かないまま文が閉じたらその行で終わる。
fn 項目の終わりの行を求める(コード行一覧: &[String], 属性の添字: usize) -> usize {
    let mut 深さ = 0usize;
    let mut 開いたことがある = false;
    for (添字, 行) in コード行一覧.iter().enumerate().skip(属性の添字) {
        for 文字 in 行.chars() {
            match 文字 {
                '{' => {
                    深さ += 1;
                    開いたことがある = true;
                }
                '}' => {
                    深さ = 深さ.saturating_sub(1);
                    if 開いたことがある && 深さ == 0 {
                        return 添字 + 1;
                    }
                }
                ';' if !開いたことがある => return 添字 + 1,
                _ => {}
            }
        }
    }
    コード行一覧.len()
}
