//! 関数の署名の境界を読んだ値と、名前がそこに現れるかの問い。関数の署名の境界とは、関数の型引数の並びの境界と、関数の `where` 句と、引数の `impl Trait` の3か所のことである。
//! 受け取るのは名前と型引数の並び(`fn` の後ろから引数の丸括弧の手前まで)と引数の一覧と引数の並びより後ろの表記、返すのはこの値である。
//! この3か所を照らすのは、関数の境界を通すと、その関数が境界の型の値を書き換えられるためである(`fn 変える(mut 相手: impl DerefMut<Target = Self>) { 相手.0 = 1; }`)。これは、受け手と引数の型の字面に可変参照を持たない。
//! 戻り値の型(`-> impl Iterator<Item = Self>`)は照らさない。戻り値は関数が作る値であり、呼び出し側の値を書き換える口を与えないためである。
//! 閉じた名前も `Self` と対象の型の名前も、可変の借用を与えるトレイト(`impl_syntax/mutable_borrow_trait.rs`)の型引数の中だけで照らす。理由は `_doc/設計/設計オントロジー.md` 2.5.3 が持つ。

use super::super::identifier_boundary::識別子として現れる位置一覧;
use super::super::impl_syntax::{可変の借用を与えるトレイトの型引数が名前を指すか, 実装の境界, 宣言と境界の句に分ける};
use super::super::line_matching::先頭の型引数を分ける;
use super::super::parameter_form::要素の終わり;

/// 関数の署名の境界。型引数の並びの境界と `where` 句と、引数の `impl Trait` の表記を持つ。
pub struct 関数の署名の境界 {
    型引数と境界の句: 実装の境界,
    引数のimplの境界: String, // 引数の型の中の `impl` から、その要素の終わりまでの表記を空白で繋いだもの
}

impl 関数の署名の境界 {
    pub(super) fn 読む(名前と型引数: &str, 引数一覧: &[String], 引数の並びより後ろ: &str) -> Self {
        let 型引数 = 名前と型引数.find('<').map_or("", |位置| 先頭の型引数を分ける(&名前と型引数[位置..]).0);
        let 引数のimplの境界 = 引数一覧
            .iter()
            .flat_map(|引数| 識別子として現れる位置一覧(引数, "impl").into_iter().map(move |位置| &引数[位置..位置 + 要素の終わり(&引数[位置..])]))
            .collect::<Vec<_>>()
            .join(" ");
        Self {
            型引数と境界の句: 実装の境界::表記から読む(型引数, 宣言と境界の句に分ける(引数の並びより後ろ).1),
            引数のimplの境界,
        }
    }

    /// 3か所の可変の借用を与えるトレイトの型引数が、名前を指すか(`U: DerefMut<Target = 規則>`・`impl AsMut<Self>`)。
    pub fn 可変の借用で名前を指すか(&self, 名前: &str) -> bool {
        self.型引数と境界の句.可変の借用で名前を指すか(名前) || 可変の借用を与えるトレイトの型引数が名前を指すか(&self.引数のimplの境界, 名前)
    }

    /// 3か所の可変の借用を与えるトレイトの型引数が、`Self` か型名を指すか。
    pub fn 可変の借用で自分の型を指すか(&self, 型名: &str) -> bool {
        ["Self", 型名].into_iter().any(|名前| self.可変の借用で名前を指すか(名前))
    }
}

#[cfg(test)]
mod tests {
    use super::super::関数の署名を読む;

    #[test]
    fn 型引数の並びの境界とwhere句と引数のimplの境界を読み戻り値の型は読まない() {
        let 読む = |表記: &str| 関数の署名を読む(表記).map(|署名| 署名.境界);
        let 境界一覧 = [
            "fn 変える<U: DerefMut<Target = Self>>(mut 相手: U) {",
            "fn 変える(mut 相手: impl DerefMut<Target = Self>) {",
            "fn 変える<U>(mut 相手: U) where U: DerefMut<Target = Self>; fn 次(&self) {}",
        ];
        for 表記 in 境界一覧 {
            assert!(読む(表記).is_some_and(|境界| 境界.可変の借用で自分の型を指すか("規則")), "{表記}");
        }
        for 表記 in ["fn 並べる() -> impl Iterator<Item = Self> {", "fn 比べる<U: PartialEq<Self>>(&self, 相手: &U) -> bool;", "fn 使う(処理: impl FnMut(&mut Self)) {"] {
            assert!(読む(表記).is_some_and(|境界| !境界.可変の借用で自分の型を指すか("規則")), "{表記}");
        }
        assert!(読む("fn 変える<T>(&mut self) where T: DerefMut<Target = 規則> {").is_some_and(|境界| 境界.可変の借用で名前を指すか("規則")));
        for 表記 in [
            "fn 足す(&self, 値: (impl AsMut<u8>, 規則)) {",
            "fn 足す<I: IntoIterator<Item = 規則>>(&mut self, 列: I) {",
            "fn 足す(&self) -> u8; fn 次<T>(値: T) where T: DerefMut<Target = 規則> {}",
        ] {
            assert!(読む(表記).is_some_and(|境界| !境界.可変の借用で名前を指すか("規則")), "{表記}");
        }
    }
}
