//! `impl トレイト for 型名` の1件。構文検査が違反の報告に使う位置(パスと行番号)と対象の型名を持ち、正本の包括の実装かという判定を自分で答える。

use std::path::PathBuf;

use super::line_matching::クレート名;

/// `impl トレイト for 型名` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct トレイト実装型 {
    pub 型名: String,
    pub パス: PathBuf,
    pub 行番号: usize,
}

impl トレイト実装型 {
    /// 正本 `blitz_design` の `marker.rs` が持つ、標準の `Result<T, E>` への `M結果` の包括の実装か。標準の `Result` は `crates` 配下に定義を持たないため、この1箇所だけを定義をたどらない特例にする。
    /// 判定にファイルのパスを使うのは、対象の型名が `Result` であることだけで判定すると、`Result` という名前の独自の `struct` が構文の法則を迂回できるためである。
    pub fn 正本の標準resultへの包括の実装か(&self) -> bool {
        クレート名(&self.パス) == "blitz_design" && self.パス.file_name().is_some_and(|名前| 名前 == "marker.rs") && self.型名 == "Result"
    }
}
