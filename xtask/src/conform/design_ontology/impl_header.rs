//! `impl` と `trait` の見出しから、型引数内の式ブロックと区別して本体の範囲を読む。本体の直下の関数の署名を読む工程へ渡す本体の文字列もここが組む。

use super::declaration_brackets::{最上位で開いた括弧, 見出しの括弧の深さ};
use super::line_matching::implの予約語より後ろ;

/// 宣言の見出しと本体の範囲。表記は見出しの行の先頭から本体を開く `{` までを空白で繋いだものであり、行番号は0始まりの添字である。
pub struct 実装の見出し {
    pub 表記: String,
    pub 本体の開始行: usize,
    pub 本体の終了行: usize,
    本体の開きの位置: usize, // 本体の開始行の中の、本体を開く `{` のバイト位置
}

impl 実装の見出し {
    /// 本体を開く `{` から閉じる `}` までを改行で繋いだ文字列。本体の直下の関数の署名を読む工程が受け取る。
    pub fn 本体の文字列(&self, 行一覧: &[String]) -> String {
        let mut 本体 = String::new();
        for (行番号, 行) in 行一覧.iter().enumerate().take(self.本体の終了行 + 1).skip(self.本体の開始行) {
            let 読む部分 = if 行番号 == self.本体の開始行 {
                行.get(self.本体の開きの位置..).unwrap_or_default()
            } else {
                行.as_str()
            };
            本体.push_str(読む部分);
            本体.push('\n');
        }
        本体
    }
}

/// 開始の行が `impl`(前に `unsafe` があってもよい)で始まるなら、その見出しと本体の範囲を読む。
pub fn implの見出しを読む(行一覧: &[String], 開始: usize) -> Option<実装の見出し> {
    implの予約語より後ろ(行一覧.get(開始)?)?;
    宣言の見出しを読む(行一覧, 開始)
}

/// 開始の行から本体を開く `{` までを読み、本体が閉じる行を求める。`impl` の見出しと `trait` の見出しが共有する。
pub fn 宣言の見出しを読む(行一覧: &[String], 開始: usize) -> Option<実装の見出し> {
    let mut 括弧 = 見出しの括弧の深さ::default();
    let mut 表記 = String::new();
    for (行番号, 行) in 行一覧.iter().enumerate().skip(開始) {
        for (文字位置, 文字) in 行.char_indices() {
            表記.push(文字);
            if 括弧.一文字読む(文字) == 最上位で開いた括弧::本体の波括弧 {
                return Some(実装の見出し {
                    表記,
                    本体の開始行: 行番号,
                    本体の終了行: 本体が閉じる行(行一覧, 行番号, 文字位置)?,
                    本体の開きの位置: 文字位置,
                });
            }
        }
        表記.push(' ');
    }
    None
}

fn 本体が閉じる行(行一覧: &[String], 開始行: usize, 開きの位置: usize) -> Option<usize> {
    let mut 深さ = 0usize;
    for (行番号, 行) in 行一覧.iter().enumerate().skip(開始行) {
        let 読む部分 = if 行番号 == 開始行 { 行.get(開きの位置..)? } else { 行.as_str() };
        for 文字 in 読む部分.chars() {
            match 文字 {
                '{' => 深さ += 1,
                '}' => {
                    深さ = 深さ.checked_sub(1)?;
                    if 深さ == 0 {
                        return Some(行番号);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::implの見出しを読む;

    #[test]
    #[allow(clippy::expect_used)]
    fn 定数式が閉じた行からでも実装本体の終了までを読む() {
        let 行一覧 = ["impl<T> 規則<T> where T: 境界<{", "1 }> {", "fn 変える(&mut self) {}", "}"].map(str::to_string);
        let 見出し = implの見出しを読む(&行一覧, 0).expect("実装本体を読む");
        assert_eq!(見出し.本体の開始行, 1);
        assert_eq!(見出し.本体の終了行, 3);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn unsafeの付いた実装の見出しも読み本体の文字列は開く波括弧から始まる() {
        let 行一覧 = ["unsafe impl 変更可能 for 規則 {", "    fn 変える(&mut self) {}", "}"].map(str::to_string);
        let 見出し = implの見出しを読む(&行一覧, 0).expect("unsafe の実装を読む");
        assert_eq!(見出し.本体の終了行, 2);
        assert!(見出し.本体の文字列(&行一覧).starts_with('{'));
    }
}
