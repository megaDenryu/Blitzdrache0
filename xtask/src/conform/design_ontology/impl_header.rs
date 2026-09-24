//! `impl` の見出しを、型引数内の式ブロックと区別して本体を開く `{` まで読む。受け取るのはコードだけの行の一覧と書き出しの行、返すのは見出しの表記か、`impl` の見出しでないときの無しである。

use super::declaration_brackets::{最上位で開いた括弧, 見出しの括弧の深さ};
use super::impl_syntax::実装の見出しの構文;
use super::line_matching::implの予約語より後ろ;

/// 実装の見出し。表記は見出しの行の先頭から本体を開く `{` までを空白で繋いだものである。
pub struct 実装の見出し {
    pub 表記: String,
}

impl 実装の見出し {
    /// 見出しの表記を、実装の種類と対象の型と型引数の名前へ分けて読む。`impl` の見出しでなければ無い。
    pub fn 構文を読む(&self) -> Option<実装の見出しの構文> {
        実装の見出しの構文::読む(&self.表記)
    }
}

/// 開始の行が `impl`(前に同じ行の属性と `unsafe` があってもよい)で始まるなら、本体を開く `{` までの見出しを読む。ファイルの最後まで `{` に届かなければ無い。
pub fn implの見出しを読む(行一覧: &[String], 開始: usize) -> Option<実装の見出し> {
    implの予約語より後ろ(行一覧.get(開始)?)?;
    宣言の見出しを読む(行一覧, 開始)
}

// 開始の行から本体を開く `{` までを読む。型引数の定数式の波括弧(`境界<{ 1 }>`)は本体の波括弧と区別する。
fn 宣言の見出しを読む(行一覧: &[String], 開始: usize) -> Option<実装の見出し> {
    let mut 括弧 = 見出しの括弧の深さ::default();
    let mut 表記 = String::new();
    for 行 in 行一覧.iter().skip(開始) {
        for 文字 in 行.chars() {
            表記.push(文字);
            if 括弧.一文字読む(文字) == 最上位で開いた括弧::本体の波括弧 {
                return Some(実装の見出し { 表記 });
            }
        }
        表記.push(' ');
    }
    None
}

#[cfg(test)]
mod tests {
    use super::implの見出しを読む;

    #[test]
    #[allow(clippy::expect_used)]
    fn 定数式が閉じた行からでも本体を開く波括弧までを読む() {
        let 行一覧 = ["impl<T> 規則<T> where T: 境界<{", "1 }> {", "fn 変える(&mut self) {}", "}"].map(str::to_string);
        let 見出し = implの見出しを読む(&行一覧, 0).expect("実装の見出しを読む");
        assert_eq!(見出し.表記, "impl<T> 規則<T> where T: 境界<{ 1 }> {");
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn unsafeの付いた実装の見出しも読む() {
        let 行一覧 = ["unsafe impl 変更可能 for 規則 {", "    fn 変える(&mut self) {}", "}"].map(str::to_string);
        let 構文 = implの見出しを読む(&行一覧, 0).and_then(|見出し| 見出し.構文を読む()).expect("unsafe の実装を読む");
        assert_eq!(構文.トレイトの表記(), Some("変更可能"));
        assert_eq!(構文.対象.名前(), "規則");
    }
}
