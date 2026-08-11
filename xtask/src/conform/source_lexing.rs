//! Rustの原文を、コード・コメント・文字列リテラルの断片へ分ける字句の走査。
//! 受け取るのは原文、返すのは区分と開始行の付いた断片の並びである。
//!
//! 「その語が本当に文字列リテラルの中に在るか」を2つの検査が同じ規則で判定するため、走査をここ1箇所に置く。
//! 引用符の数え上げでは判定が成立しない。コメントの中の引用符が区間を開き、逃がし記号の`\"`が区間を閉じ、
//! 生文字列の`r#"..."#`は逃がし記号を持たない。どれも「コメントでの言及」を出現と数え違える原因になる。
//!
//! 中身は区切りの記号を除いた内側である。生文字列と通常文字列が同じ中身を持つようにするためであり、
//! これがないと同じファイル名が書き方の違いで別の綴りとして数えられる。

mod code_lines;
#[cfg(test)]
mod tests;
mod token_read;

pub use code_lines::コードだけの行一覧;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 字句の区分 {
    コード,
    行コメント,
    ブロックコメント,
    文字列リテラル,
}

#[derive(Debug, PartialEq, Eq)]
pub struct 字句の断片 {
    pub 区分: 字句の区分,
    pub 開始行: usize,
    pub 中身: String,
}

pub fn 字句へ分ける(内容: &str) -> Vec<字句の断片> {
    let 文字一覧: Vec<char> = 内容.chars().collect();
    let mut 断片一覧 = Vec::new();
    let mut 溜めたコード = String::new();
    let mut コードの開始行 = 1usize;
    let mut 行番号 = 1usize;
    let mut 位置 = 0usize;
    while let Some(&文字) = 文字一覧.get(位置) {
        let Some(読み) = token_read::区切りを読む(&文字一覧, 位置) else {
            if 溜めたコード.is_empty() {
                コードの開始行 = 行番号;
            }
            溜めたコード.push(文字);
            if 文字 == '\n' {
                行番号 += 1;
            }
            位置 += 1;
            continue;
        };
        溜めたコードを吐き出す(&mut 断片一覧, &mut 溜めたコード, コードの開始行);
        let 断片の開始行 = 行番号;
        行番号 += 読み.中身.matches('\n').count();
        断片一覧.push(字句の断片 {
            区分: 読み.区分,
            開始行: 断片の開始行,
            中身: 読み.中身,
        });
        位置 = 読み.次の位置;
        コードの開始行 = 行番号;
    }
    溜めたコードを吐き出す(&mut 断片一覧, &mut 溜めたコード, コードの開始行);
    断片一覧
}

pub fn 文字列リテラル一覧(内容: &str) -> Vec<字句の断片> {
    字句へ分ける(内容)
        .into_iter()
        .filter(|断片| 断片.区分 == 字句の区分::文字列リテラル)
        .collect()
}

fn 溜めたコードを吐き出す(断片一覧: &mut Vec<字句の断片>, 溜めたコード: &mut String, 開始行: usize) {
    if 溜めたコード.is_empty() {
        return;
    }
    断片一覧.push(字句の断片 {
        区分: 字句の区分::コード,
        開始行,
        中身: std::mem::take(溜めたコード),
    });
}
