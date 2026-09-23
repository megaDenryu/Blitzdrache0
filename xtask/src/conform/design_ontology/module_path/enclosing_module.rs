//! ファイルの行ごとに、その行の頭を囲む波括弧付きのモジュール(`mod 名 { … }`)の名前の並びを求める純粋な関数。
//! トレイトの宣言のモジュールパスと実装の位置のモジュールパスを、ファイルから推定したモジュールパスの下へ繋ぐために使う。
//! 数えるのはコードだけの行の波括弧であり、`mod 名 {` の行の最初の `{` だけをそのモジュールの本体とする。関数・実装・トレイトの本体の波括弧はモジュールを作らないため、並びに入れない。

use super::super::declaration_prefix::属性と可視性を読み飛ばす;
use super::super::line_matching::先頭の識別子;

/// 行ごとの、その行の頭を囲む波括弧付きのモジュールの名前の並び(外側から順)。
pub fn 行ごとの囲むモジュールの名前一覧(行一覧: &[String]) -> Vec<Vec<String>> {
    let mut 開いた波括弧: Vec<Option<String>> = Vec::new(); // 開いている波括弧ごとの、それを本体に持つモジュールの名前
    let mut 一覧 = Vec::with_capacity(行一覧.len());
    for 行 in 行一覧 {
        一覧.push(開いた波括弧.iter().flatten().cloned().collect());
        let mut モジュール名 = 波括弧付きのモジュールの見出しの名前(行);
        for 文字 in 行.chars() {
            match 文字 {
                '{' => 開いた波括弧.push(モジュール名.take()),
                '}' => {
                    開いた波括弧.pop();
                }
                _ => {}
            }
        }
    }
    一覧
}

// `mod 名 {` の行(前に同じ行の属性と可視性があってもよい)なら、そのモジュールの名前。
fn 波括弧付きのモジュールの見出しの名前(行: &str) -> Option<String> {
    let 後ろ = 属性と可視性を読み飛ばす(行).strip_prefix("mod ")?.trim_start();
    let 名前 = 先頭の識別子(後ろ);
    let 名前の後ろ = 後ろ.get(名前.len()..)?.trim_start();
    (!名前.is_empty() && 名前の後ろ.starts_with('{')).then_some(名前)
}

#[cfg(test)]
mod tests {
    use super::行ごとの囲むモジュールの名前一覧;

    #[test]
    fn 入れ子のモジュールの名前を外側から並べ関数の本体の波括弧は数えない() {
        let 行一覧 = ["mod 丙 {", "    fn f() {", "        let _ = 1;", "    }", "    pub(crate) mod 甲 {", "        pub trait 変更 {}", "    }", "}", "trait 外 {}"].map(str::to_string);
        let 一覧 = 行ごとの囲むモジュールの名前一覧(&行一覧);
        assert!(一覧[0].is_empty());
        assert_eq!(一覧[2], ["丙"]);
        assert_eq!(一覧[5], ["丙", "甲"]);
        assert!(一覧[8].is_empty());
    }
}
