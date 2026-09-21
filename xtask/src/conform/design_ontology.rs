//! 設計オントロジーの構文検査。`blitz_design` のトレイトを実装した型が、コンパイラが強制しない設計規則を守っているかを、
//! `crates` 配下の各クレートの `src` の下の全 `.rs` を横断して検査する。受け取るのは無し、返すのは違反一覧か、ソースを読めなかった理由である。
//!
//! 検査する規則: `Mコマンド` は enum である。`M規則` は struct である。`M結果` を実装する型は enum である(正本 `blitz_design` の `marker.rs` が持つ完全に修飾した `std::result::Result` への包括の実装だけが定義をたどらない特例)。`M不変データ` またはデータの役割(`Mコマンド`・`Mイベント`・`M規則`・`M状態`・`M入力`・`M観測`)を実装する型の定義は
//! 参照・生ポインタ・内部可変性を持たない(役割の実装から型の定義をたどってその定義に当てる。`impl M不変データ for` の置き場所は問わず、同じ型が複数の役割を持っても違反は1回だけ報告する)。
//! `M不変データ` を実装する型(データの役割を実装する型をすべて含む)の固有の `impl` とトレイトの実装は `&mut self` メソッドを持たない。`MParameter` を実装する型の定義は `Option<` のフィールドを持たない(関数境界の役割。型の別名を通した `Option` と別の構造体に埋めた `Option` は見ない)。定義が見つからない実装は違反にする。
//! `crates` 配下の `src` のすべての `mod` の宣言について、宣言された論理のモジュール構造と、対象の物理ファイルから得られる物理のモジュール構造が一致する(`module_declaration.rs` が宣言を抽出し、`module_structure_assertion.rs` が一致を確かめる)。
//! 設計解釈マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形に固定する。マーカーの名前を含む正規形でない実装の行(再公開したパスの経由・`::blitz_design::` の絶対パス・`blitz_design :: M不変データ` のようなパスの中の空白)と、
//! `blitz_design` を別名で取り込むこと(`use ... as`)と、`blitz_design` の外で `blitz_design` を公開の `use` で再公開することと、ファイルの中の `mod 名 { ... }` の中にマーカーの実装を置くこと(型の定義をファイル単位で探すため)は違反にする。
//! 同じファイルに同名の定義が複数あるときは一意に決まらないとして違反にする。同じ型が同じ軸の排他の分類(`M不変エンティティ` と `M可変エンティティ`、`MParameter` と `MOptions`)を同時に名乗ることは違反にする。
//! 検査しない規則(コンパイラが強制する): `Mコマンド: M不変データ` 等の上位トレイトの関係、`M不変データ` の `Clone`、関数の役割(境界付きの newtype)の型引数の境界、`遷移成功結果` の型引数の境界。
//! 保証範囲: 検査は Rust の型意味論でなく構文パターン(`impl トレイト for 型` の行と `struct`/`enum` の定義ブロック)に対して行う。型の同一性は
//! 「標準的なファイル配置(`src/a/b.rs` → `a::b`、`mod.rs`・`lib.rs`)から推定したモジュールパス + 型名」であり、実装のファイルと同じファイルの定義、無ければそのファイルの `use` 行(`crate::`・`super::`・`self::` と1段の波括弧の群)から取り込み元のモジュールパスを求めて、
//! そのモジュールパスの定義を採る。`use` が無ければ全体で1つだけの同名の定義を採り、2つ以上なら違反にする。実装対象の型の `use ... as` の別名は取り込み元を求められない違反にする。
//! 固有の `impl` の探索も同じ規則で定義に属するファイルだけを見る。入れ子の波括弧の `use`、glob の取り込み、型引数の境界が入れ子の `<` を含むジェネリックな `impl`、フィールドの型の中に間接的に含まれる内部可変性は保証範囲の外である。
//! `#[path = "..."] mod` は、物理と論理のモジュール構造の一致を確かめるため、型の同一性の推定をそのまま使える(日本語のモジュールは rustc が E0754 で既定の探索を拒むため、この属性を必ず持つ)。

mod exclusive_classification_assertion;
#[cfg(test)]
mod exclusive_classification_tests;
#[cfg(test)]
mod existence_marker_form_tests;
#[cfg(test)]
mod japanese_module_hierarchy_tests;
mod line_matching;
#[cfg(test)]
mod marker_canonical_form_tests;
mod marker_form_assertion;
#[cfg(test)]
mod marker_form_tests;
mod module_declaration;
mod module_declaration_extract;
#[cfg(test)]
mod module_declaration_tests;
mod module_path;
#[cfg(test)]
mod module_path_tests;
mod module_structure_assertion;
#[cfg(test)]
mod module_structure_assertion_tests;
#[cfg(test)]
mod mutable_self_law_tests;
mod parameter_assertion;
#[cfg(test)]
mod parameter_assertion_tests;
#[cfg(test)]
mod process_marker_form_tests;
#[cfg(test)]
mod result_marker_form_tests;
mod syntax_assertion;
mod syntax_checker;
mod syntax_patterns;
#[cfg(test)]
mod tests;
mod trait_implementation;
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
use module_structure_assertion::モジュール構造の一致検査;
use syntax_checker::クレート構文検査;

pub fn 全ファイルを検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut ソース一覧 = Vec::new();
    let mut モジュール構造の違反一覧 = Vec::new();
    for パス in file_scan::対象ファイル一覧を集める(&["crates"], &["rs"])?.into_iter().filter(|パス| srcの下か(パス)) {
        let 内容 = std::fs::read_to_string(&パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(&パス, 誤り))?;
        モジュール構造の違反一覧.extend(モジュール構造の一致検査::生成する(パス.clone(), &内容).違反一覧());
        ソース一覧.push((パス, コードだけの行一覧(&内容)));
    }
    let mut 違反一覧 = クレート構文検査::生成する(ソース一覧)
        .すべてのコマンドが列挙型であること()
        .すべての規則が構造体であること()
        .すべての結果が列挙型であること()
        .すべての純粋データが参照と内部可変性を持たないこと()
        .すべての不変データが可変参照メソッドを持たないこと()
        .すべての引数オブジェクトが任意の値を持たないこと()
        .排他の分類を同時に名乗っていないこと()
        .設計解釈マーカーを別名で取り込んでいないこと()
        .設計解釈マーカーの実装が正規形であること()
        .設計解釈マーカーを再公開していないこと()
        .設計解釈マーカーの実装が波括弧付きのモジュールの中に無いこと()
        .違反一覧();
    違反一覧.extend(モジュール構造の違反一覧);
    Ok(違反一覧)
}

fn srcの下か(パス: &Path) -> bool {
    パス.components().any(|部品| matches!(部品, Component::Normal(名前) if 名前 == "src"))
}
