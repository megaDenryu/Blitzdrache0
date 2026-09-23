//! `impl トレイト for 型` の1件。構文検査が違反の報告に使う位置(パスと行番号)と、対象の型名(定義をたどる鍵)と対象の型の表記そのもの(パスの修飾と型引数を含む)を持ち、
//! 正本の包括の実装かという判定と、実装の見出しの行の位置のモジュール(`module_path/enclosing_module.rs`)を自分で答える。

use std::path::PathBuf;

use super::line_matching::パスの最後の名前;
use super::marker_canonical_file::設計解釈マーカーの正本のファイルか;
use super::module_path::モジュールパス;

/// `impl トレイト for 型` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct トレイト実装型 {
    pub 型名: String,           // 対象の型の表記の先頭の識別子。定義をたどる鍵であり、`std::result::Result<T, E>` なら `std` になる
    pub 対象の型の表記: String, // `実装の見出しの構文` が読んだ対象の型の表記(参照と `Pin` と囲みの丸括弧を外したもの。`std::result::Result<T, E>`)
    pub パス: PathBuf,
    pub 行番号: usize,
}

impl トレイト実装型 {
    /// 正本 `blitz_design` の `marker.rs` が持つ、完全に修飾した標準の `std::result::Result<T, E>` への `M結果` の包括の実装か。標準の `Result` は `crates` 配下に定義を持たないため、この1箇所だけを定義をたどらない特例にする。
    /// 対象の型の表記が `std::result::Result` で始まることで判定するのは、型名が `Result` であることや正本のファイルにあることでは、同名の独自の型(正本のファイルに置いたものを含む)が特例を偽装できるためである。
    pub fn 正本の標準resultへの包括の実装か(&self) -> bool {
        設計解釈マーカーの正本のファイルか(&self.パス) && (self.対象の型の表記 == "std::result::Result" || self.対象の型の表記.starts_with("std::result::Result<"))
    }

    /// 対象の型の表記のパスの最後の名前(`crate::a::規則<T>` なら `規則`)。名前で実装を集める検査が、この名前を種にする。
    pub fn 対象の型のパスの最後の名前(&self) -> &str {
        パスの最後の名前(&self.対象の型の表記)
    }

    /// 実装の見出しの行の位置のモジュール(ファイルから推定したモジュールパス + 見出しの行を囲む `mod 名 { … }` の並び)。定義が一意に決まらない実装の型の同一性をこのモジュールで数える。
    /// 実装のファイルが走査したソースに無ければ、ファイルから推定したモジュールパスである。
    pub fn 位置のモジュール(&self, ソース一覧: &[(PathBuf, Vec<String>)]) -> モジュールパス {
        let ファイルのモジュール = モジュールパス::ファイルのパスから求める(&self.パス);
        let Some((_, 行一覧)) = ソース一覧.iter().find(|(パス, _)| *パス == self.パス) else {
            return ファイルのモジュール;
        };
        let 位置 = ファイルのモジュール.行ごとの字句位置一覧(行一覧).into_iter().nth(self.行番号.saturating_sub(1));
        位置.map_or(ファイルのモジュール, |位置| 位置.モジュール)
    }
}
