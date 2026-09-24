//! 可変の借用を与えるトレイト(`DerefMut`・`BorrowMut`・`AsMut`・`IndexMut`)が表記に現れるかと、その型引数が名前を指すかを答える純粋な関数。受け取るのは表記と名前、返すのは真偽である。
//! この4つを区別するのは、その境界を満たす値が、境界の型引数(`Target`・`Borrowed`・`T`・`Output`)の型への可変参照を与えるためである。値で受けた `mut self` と、関数の型引数で受けた値からも、その型の値を書き換えられる。
//! 自己変更の検査は、関数の署名の境界の `Self` と、対象の表記の可変の借用をこの規則で見る。`PartialEq<Self>`・`Iterator<Item = Self>`・`FnMut(&mut Self)` の境界は値を比べるか作るか受け取る処理であり、`Self` の値への可変参照を与えないため数えない。
//! これは名前による近似であり、この4つを上位トレイトに持つ別のトレイトの宣言を通した境界は捕まえない。

use super::super::identifier_boundary::{識別子として現れるか, 識別子として現れる位置一覧};
use super::super::line_matching::先頭の型引数を分ける;

const 可変の借用を与えるトレイトの名前一覧: [&str; 4] = ["DerefMut", "BorrowMut", "AsMut", "IndexMut"];

/// 表記に、可変の借用を与えるトレイトの名前が識別子の境界で現れるか(`Box<dyn DerefMut<Target = 規則>>`)。
pub fn 可変の借用を与えるトレイトが現れるか(表記: &str) -> bool {
    可変の借用を与えるトレイトの名前一覧.iter().any(|トレイト| 識別子として現れるか(表記, トレイト))
}

/// 表記の中の可変の借用を与えるトレイトの型引数(直後の山括弧の内側)に、名前が識別子の境界で現れるか(`DerefMut<Target = Self>` の `Self`)。
pub fn 可変の借用を与えるトレイトの型引数が名前を指すか(表記: &str, 名前: &str) -> bool {
    可変の借用を与えるトレイトの名前一覧.iter().any(|トレイト| {
        識別子として現れる位置一覧(表記, トレイト)
            .into_iter()
            .any(|位置| 識別子として現れるか(先頭の型引数を分ける(表記[位置 + トレイト.len()..].trim_start()).0, 名前))
    })
}

#[cfg(test)]
mod tests {
    use super::{可変の借用を与えるトレイトが現れるか, 可変の借用を与えるトレイトの型引数が名前を指すか};

    #[test]
    fn 可変の借用を与えるトレイトの型引数の名前だけを当てる() {
        for 表記 in ["DerefMut<Target = Self>", "std::borrow::BorrowMut<Self>", "AsMut<Vec<Self>>", "IndexMut<usize, Output = Self>"] {
            assert!(可変の借用を与えるトレイトの型引数が名前を指すか(表記, "Self"), "{表記}");
        }
        for 表記 in ["PartialEq<Self>", "Iterator<Item = Self>", "FnMut(&mut Self)", "DerefMut + From<Self>", "MyDerefMut<Target = Self>"] {
            assert!(!可変の借用を与えるトレイトの型引数が名前を指すか(表記, "Self"), "{表記}");
        }
        assert!(可変の借用を与えるトレイトが現れるか("Box<dyn DerefMut<Target = 規則>>") && !可変の借用を与えるトレイトが現れるか("Box<dyn Deref<Target = 規則>>"));
    }
}
