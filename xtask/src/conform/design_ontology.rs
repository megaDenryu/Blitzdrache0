//! 設計オントロジーの構文検査。`blitz_design` のトレイトを実装した型が、コンパイラが強制しない設計規則を守っているかを、
//! `crates` 配下の各クレートの `src` の下の全 `.rs` を横断して検査する。受け取るのは無し、返すのは違反一覧か、ソースを読めなかった理由である。
//!
//! 検査する規則: `Mコマンド` は enum である。`M規則` は struct である。`MDTO` と `Mイベント` の定義は参照・生ポインタ・内部可変性を持たない。
//! `Mコマンド`・`Mイベント`・`M規則` の固有の `impl` は `&mut self` メソッドを持たない。定義が見つからない実装は違反にする。
//! 検査しない規則(コンパイラが強制する): `M状態` と `M入力` の `MDTO` 実装、`MDTO` と `Mイベント` の `Clone`、関数型の型引数の境界。
//! 保証範囲: 検査は Rust の型意味論でなく構文パターン(`impl トレイト for 型` の行と `struct`/`enum` の定義ブロック)に対して行う。同名の型が複数のクレートにあるときは
//! 実装と同じファイルの定義を採り、決まらなければ違反にする。ジェネリックな `impl` や、フィールドの型の中に間接的に含まれる内部可変性は保証範囲の外である。

mod syntax_assertion;
mod syntax_checker;
mod syntax_patterns;
#[cfg(test)]
mod tests;
mod type_definition;

use std::path::{Component, Path};

use super::error::規約検査の破れ;
use super::source_lexing::コードだけの行一覧;
use super::violation::違反;
use crate::file_scan;
use syntax_checker::クレート構文検査;

pub fn 全ファイルを検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut ソース一覧 = Vec::new();
    for パス in file_scan::対象ファイル一覧を集める(&["crates"], &["rs"])?.into_iter().filter(|パス| srcの下か(パス)) {
        let 内容 = std::fs::read_to_string(&パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(&パス, 誤り))?;
        ソース一覧.push((パス, コードだけの行一覧(&内容)));
    }
    Ok(クレート構文検査::生成する(ソース一覧)
        .すべてのコマンドが列挙型であること()
        .すべての規則が構造体であること()
        .すべてのmdtoが純粋データ規約を満たすこと()
        .すべてのイベントが純粋データ規約を満たすこと()
        .すべてのコマンドが可変参照メソッドを持たないこと()
        .すべてのイベントが可変参照メソッドを持たないこと()
        .すべての規則が可変参照メソッドを持たないこと()
        .違反一覧())
}

fn srcの下か(パス: &Path) -> bool {
    パス.components().any(|部品| matches!(部品, Component::Normal(名前) if 名前 == "src"))
}
