//! 決まった文字列から1つ選ぶ引数の、選べる文字列1件。担当するのは文字列とその説明の対を保つことだけである。
//!
//! 裸の対で持ち回らないのは、文字列と説明がどちらも`&'static str`であり、順を取り違えても型が通るためである。
//! 取り違えると選択肢の一覧に文字列が説明として並び、読み手はどれを選べばよいか分からないまま合格する。

/// 排他の選択肢の1件。文字列はそのままコマンド行へ渡る語であり、説明は選ぶ人が読む1行である。
#[derive(Clone, Copy)]
pub(crate) struct 選択肢 {
    文字列: &'static str,
    説明: &'static str,
}

impl 選択肢 {
    pub(crate) const fn 生成する(文字列: &'static str, 説明: &'static str) -> Self {
        Self { 文字列, 説明 }
    }

    /// コマンド行へそのまま渡る語。生の文字列へ戻るのはコマンド行を組み立てる境界だけである。
    pub(crate) fn 文字列(self) -> &'static str {
        self.文字列
    }

    pub(crate) fn 説明(self) -> &'static str {
        self.説明
    }
}
