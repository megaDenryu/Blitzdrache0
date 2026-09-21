//! `impl トレイト for 型` の1件。構文検査が違反の報告に使う位置(パスと行番号)と、対象の型名(定義をたどる鍵)と対象の型の表記そのもの(パスの修飾と型引数を含む)を持ち、正本の包括の実装かという判定を自分で答える。

use std::path::PathBuf;

use super::marker_canonical_file::設計解釈マーカーの正本のファイルか;

/// `impl トレイト for 型` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct トレイト実装型 {
    pub 型名: String,           // 対象の型の表記の先頭の識別子。定義をたどる鍵であり、`std::result::Result<T, E>` なら `std` になる
    pub 対象の型の表記: String, // ` for ` の後ろの表記そのもの(`std::result::Result<T, E>`)
    pub パス: PathBuf,
    pub 行番号: usize,
}

impl トレイト実装型 {
    /// 正本 `blitz_design` の `marker.rs` が持つ、完全に修飾した標準の `std::result::Result<T, E>` への `M結果` の包括の実装か。標準の `Result` は `crates` 配下に定義を持たないため、この1箇所だけを定義をたどらない特例にする。
    /// 対象の型の表記が `std::result::Result` で始まることで判定するのは、型名が `Result` であることや正本のファイルにあることでは、同名の独自の型(正本のファイルに置いたものを含む)が特例を偽装できるためである。
    pub fn 正本の標準resultへの包括の実装か(&self) -> bool {
        設計解釈マーカーの正本のファイルか(&self.パス) && (self.対象の型の表記 == "std::result::Result" || self.対象の型の表記.starts_with("std::result::Result<"))
    }
}
