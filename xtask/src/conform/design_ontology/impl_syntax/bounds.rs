//! 実装の見出しの型引数の並びの境界と `where` 句を読んだ値と、名前がそのどこに当たるかの問い。受け取るのは型引数の並び(山括弧の内側)と `where` 句の表記、返すのはこの値である。
//! 自己変更の検査は、マーカーの型の名前が対象の表記に現れる実装に加えて、境界でその型を指す実装(`impl<T: DerefMut<Target = 規則>> 変更 for Vec<T>`・`where T: BorrowMut<規則>`)を集める。境界を通すと、本体がマーカーの型の値を書き換えられるためである。
//! 型引数の並びから照らすのは、各引数の名前の後ろの `:` より後ろ(境界と定数の引数の型)だけであり、型引数の名前そのものは照らさない。実装自身の型引数と同じ名前は、その実装の中では型引数を指すため、`where` 句でも当たらない(`impl<状態: M状態> 包み<状態> where 状態: Clone`)。
//! これは名前による近似であり、境界を別のトレイトの宣言か走査範囲の外のトレイトで包んだ形(`trait 取れる: DerefMut<Target = 規則> {}` を経由した `T: 取れる`)は捕まえない。

use super::super::declaration_brackets::最上位のカンマで分ける;
use super::super::identifier_boundary::識別子として現れるか;
use super::super::self_modification_evidence::名前が当たった場所;

/// 実装の見出しの型引数の並びの境界と `where` 句の表記。
pub struct 実装の境界 {
    型引数の並びの境界: String, // 型引数ごとの名前の後ろの `:` より後ろの表記を空白で繋いだもの(`impl<T: Into<地点>, const N: usize>` なら `Into<地点> usize`)
    境界の句: String,           // 山括弧の外の `where` の後ろから本体を開く `{` の手前までの表記。`where` 句が無ければ空である
}

impl 実装の境界 {
    pub(super) fn 表記から読む(型引数: &str, 境界の句: &str) -> Self {
        Self {
            型引数の並びの境界: 最上位のカンマで分ける(型引数).into_iter().filter_map(|引数| 引数.split_once(':')).map(|(_, 境界)| 境界.trim()).collect::<Vec<_>>().join(" "),
            境界の句: 境界の句.to_string(),
        }
    }

    /// 名前が型引数の並びの境界か `where` 句のどこに識別子の境界で現れるか。名前が実装自身の型引数の名前なら当たらない。
    pub fn 名前が当たった場所(&self, 名前: &str, 型引数の名前一覧: &[String]) -> Option<名前が当たった場所> {
        if 型引数の名前一覧.iter().any(|型引数| 型引数 == 名前) {
            return None;
        }
        [(&self.型引数の並びの境界, 名前が当たった場所::型引数の並びの境界), (&self.境界の句, 名前が当たった場所::境界の句)]
            .into_iter()
            .find_map(|(表記, 場所)| 識別子として現れるか(表記, 名前).then_some(場所))
    }
}

#[cfg(test)]
mod tests {
    use super::super::実装の見出しの構文;
    use super::名前が当たった場所;

    // 見出しを読み、名前が境界のどこに当たったか。
    fn 当たった場所(見出し: &str, 名前: &str) -> Option<名前が当たった場所> {
        let 構文 = 実装の見出しの構文::読む(見出し)?;
        構文.境界.名前が当たった場所(名前, 構文.型引数の名前一覧())
    }

    #[test]
    fn 型引数の並びの境界と境界の句の名前を見分け型引数の名前とトレイトの型引数と対象は照らさない() {
        assert!(matches!(当たった場所("impl<T: DerefMut<Target = 規則>> 変更 for Vec<T> {", "規則"), Some(名前が当たった場所::型引数の並びの境界)));
        assert!(matches!(当たった場所("impl<T> 変更 for Vec<T> where T: BorrowMut<規則> {", "規則"), Some(名前が当たった場所::境界の句)));
        assert!(matches!(当たった場所("impl<const N: 地点> 変更 for [u8; 1] {", "地点"), Some(名前が当たった場所::型引数の並びの境界)));
        for (見出し, 名前) in [("impl<状態: M状態> 包み<状態> where 状態: Clone {", "状態"), ("impl IndexMut<地点> for 地図 {", "地点"), ("impl 変更 for Vec<規則> {", "規則")] {
            assert!(当たった場所(見出し, 名前).is_none(), "{見出し}");
        }
    }
}
