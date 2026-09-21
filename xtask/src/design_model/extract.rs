//! 設計関係の抽出の入口。`crates` 配下のソースを1度走査し、5つの抽出の規則を順に当てて抽出の結末を組む。
//!
//! 規則の順序は、節点の種類を先に現れたものへ決めるための順序である。上位トレイトの宣言(規則1)とマーカーの実装(規則2)が
//! トレイトの節点を先に置き、役割の型引数(規則5)が処理の節点を置き、残りが型の節点になる。畳みは `設計関係グラフ::生成する` が行う。
//!
//! 入口が走査を1度だけ行うのは、規則ごとに走査すると同じファイルの読み取りが5回走り、規則の間で読み取りの結果が食い違いうるためである。

mod entity_identifier;
mod function_role;
mod held_field;
mod marker_concept;
mod marker_impl;
mod ontology_scope;
mod outcome;
mod role;
mod role_usage;
mod source_group;
mod struct_declaration;
mod supertrait;
mod trait_declaration;
mod type_notation;
mod unextracted_line;

#[cfg(test)]
mod entity_identifier_tests;
#[cfg(test)]
mod function_role_tests;
#[cfg(test)]
mod held_field_tests;
#[cfg(test)]
mod marker_impl_tests;
#[cfg(test)]
mod real_crates_tests;
#[cfg(test)]
mod supertrait_tests;
#[cfg(test)]
mod test_support;

pub use outcome::抽出の結末;
pub use unextracted_line::{抽出できなかった理由, 抽出できなかった行};

use crate::conform::error::規約検査の破れ;
use outcome::抽出の成果;
use source_group::抽出対象のソース群;

/// `crates` 配下のソースから設計関係グラフを抽出する。返すのはグラフと、抽出できなかった行の一覧の対である。
pub fn 設計関係グラフを抽出する() -> Result<抽出の結末, 規約検査の破れ> {
    Ok(ソース群から抽出する(&抽出対象のソース群::crates配下から走査して生成する()?))
}

/// 走査済みのソース群から抽出する。回帰試験が、リポジトリの実物に依存せず組んだソースを与えるための口である。
fn ソース群から抽出する(ソース群: &抽出対象のソース群) -> 抽出の結末 {
    let mut 成果 = 抽出の成果::default();
    成果.概念一覧.extend(marker_concept::設計解釈マーカーの概念一覧());
    成果.併せる(supertrait::上位トレイトの宣言から抽出する(ソース群));
    成果.併せる(marker_impl::設計解釈マーカーの実装から抽出する(ソース群));
    成果.併せる(function_role::関数の役割の型引数から抽出する(ソース群));
    成果.併せる(entity_identifier::エンティティの識別子の関連型から抽出する(ソース群));
    成果.併せる(held_field::構造体のフィールドから抽出する(ソース群));
    成果.結末にする()
}
