//! 1つのファイルの中で、実装の本体の直下(実装の本体を開く `{` の中で、他の括弧に入っていない位置)から始まる行の集まり。受け取るのはコードだけの行の一覧、返すのはその行の添字の集まりである。
//! 名前の閉包(`marker_name_closure.rs`)は、この位置の `type 名前 = 右辺;` を関連型の宣言として読み、項目の型の別名と違う向きの辺にする。
//! 実装の本体の中の関数の本体に書いた `type` は、関数の中の項目の型の別名であり、関連型ではないため、直下の行に数えない。

use std::collections::BTreeSet;

use super::impl_header::implの見出しを読む;

pub struct 実装の本体の直下の行一覧 {
    添字一覧: BTreeSet<usize>,
}

impl 実装の本体の直下の行一覧 {
    /// ファイルのすべての `impl` の見出しから本体の範囲を読み、本体の直下から始まる行の添字を集める。
    pub fn ファイルから集める(行一覧: &[String]) -> Self {
        let mut 添字一覧 = BTreeSet::new();
        for 見出し in (0..行一覧.len()).filter_map(|開始| implの見出しを読む(行一覧, 開始)) {
            let mut 深さ = 0usize;
            for (ずれ, 行) in 見出し.本体の文字列(行一覧).lines().enumerate() {
                if ずれ > 0 && 深さ == 1 {
                    添字一覧.insert(見出し.本体の開始行 + ずれ);
                }
                for 文字 in 行.chars() {
                    match 文字 {
                        '(' | '[' | '{' => 深さ += 1,
                        ')' | ']' | '}' => 深さ = 深さ.saturating_sub(1),
                        _ => {}
                    }
                }
            }
        }
        Self { 添字一覧 }
    }

    /// その添字の行が実装の本体の直下から始まるか。
    pub fn 含むか(&self, 添字: usize) -> bool {
        self.添字一覧.contains(&添字)
    }
}

#[cfg(test)]
mod tests {
    use super::実装の本体の直下の行一覧;

    #[test]
    fn 実装の本体の直下の行だけを集め関数の本体の中の行は集めない() {
        let 行一覧: Vec<String> = ["impl 甲 for 乙 {", "    type 状態 = 規則;", "    fn 読む() {", "        type 局所 = 規則;", "    }", "}", "type 項目 = 規則;"]
            .map(str::to_string)
            .to_vec();
        let 直下 = 実装の本体の直下の行一覧::ファイルから集める(&行一覧);
        assert_eq!((0..行一覧.len()).filter(|添字| 直下.含むか(*添字)).collect::<Vec<_>>(), vec![1, 2, 5]);
    }
}
