//! 設計オントロジーの構文検査。`blitz_design` のトレイトを実装した型が、コンパイラが強制しない設計規則を守っているかを、
//! `crates` 配下の各クレートの `src` の下の全 `.rs` を横断して検査する。受け取るのは無し、返すのは違反一覧か、ソースを読めなかった理由である。
//!
//! 検査する規則: `Mコマンド` は enum である。`M規則` は struct である。`MDTO` またはデータの役割(`Mコマンド`・`Mイベント`・`M規則`・`M状態`・`M入力`・`M設定`)を実装する型の定義は
//! 参照・生ポインタ・内部可変性を持たない(役割の実装から型の定義をたどってその定義に当てる。`impl MDTO for` の置き場所は問わず、同じ型が複数の役割を持っても違反は1回だけ報告する)。
//! `Mコマンド`・`Mイベント`・`M規則`・`M設定` の固有の `impl` は `&mut self` メソッドを持たない。定義が見つからない実装は違反にする。`crates` 配下の `src` に `#[path` の属性が無い。
//! 設計マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形に固定する。`blitz_design` を別名で取り込むこと(`use ... as`)と、ファイルの中の `mod 名 { ... }` の中にマーカーの実装を置くことは違反にする(型の定義をファイル単位で探すため)。
//! 同じファイルに同名の定義が複数あるときは一意に決まらないとして違反にする。
//! 検査しない規則(コンパイラが強制する): `Mコマンド: MDTO` 等の上位トレイトの関係、`MDTO` の `Clone`、関数の役割(境界付きの newtype)の型引数の境界、`遷移結果` の型引数の境界。
//! 保証範囲: 検査は Rust の型意味論でなく構文パターン(`impl トレイト for 型` の行と `struct`/`enum` の定義ブロック)に対して行う。型の同一性は
//! 「標準的なファイル配置(`src/a/b.rs` → `a::b`、`mod.rs`・`lib.rs`)から推定したモジュールパス + 型名」であり、実装のファイルと同じファイルの定義、無ければそのファイルの `use` 行(`crate::`・`super::`・`self::` と1段の波括弧の群)から取り込み元のモジュールパスを求めて、
//! そのモジュールパスの定義を採る。`use` が無ければ全体で1つだけの同名の定義を採り、2つ以上なら違反にする。実装対象の型の `use ... as` の別名は取り込み元を求められない違反にする。
//! 固有の `impl` の探索も同じ規則で定義に属するファイルだけを見る。入れ子の波括弧の `use`、glob の取り込み、ジェネリックな `impl`、フィールドの型の中に間接的に含まれる内部可変性、
//! `#[path = "..."] mod` によるファイルとモジュールの不一致(こちらは属性の存在自体を違反にする)は保証範囲の外である。

mod line_matching;
mod marker_form_assertion;
#[cfg(test)]
mod marker_form_tests;
mod module_path;
#[cfg(test)]
mod module_path_tests;
mod syntax_assertion;
mod syntax_checker;
mod syntax_patterns;
#[cfg(test)]
mod tests;
mod type_definition;
#[cfg(test)]
mod type_identity_tests;
mod use_resolution;
#[cfg(test)]
mod use_resolution_tests;

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
        .すべての純粋データが参照と内部可変性を持たないこと()
        .オントロジーの対象の原文にpath属性が無いこと()
        .すべてのコマンドが可変参照メソッドを持たないこと()
        .すべてのイベントが可変参照メソッドを持たないこと()
        .すべての規則が可変参照メソッドを持たないこと()
        .すべての設定が可変参照メソッドを持たないこと()
        .設計マーカーを別名で取り込んでいないこと()
        .設計マーカーの実装が波括弧付きのモジュールの中に無いこと()
        .違反一覧())
}

fn srcの下か(パス: &Path) -> bool {
    パス.components().any(|部品| matches!(部品, Component::Normal(名前) if 名前 == "src"))
}
