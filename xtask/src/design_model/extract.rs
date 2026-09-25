//! 設計関係の抽出の入口。`crates` 配下のソースを1度走査し、5つの抽出の規則を順に当てて抽出の結果を組む。
//!
//! 規則の順序は、同じ識別子へ種類の違う概念が来たときにどちらが先かを決める順序である。上位トレイトの宣言(規則1)と
//! マーカーの実装(規則2)がトレイトの節点を先に置き、役割の型引数(規則5)が処理の節点を置き、残りが型の節点になる。
//! 重複のまとめと同一性の衝突の検出は `設計関係グラフ::生成する` が行い、種類が違うものは順序で決着させずに両方を節点として残す。
//! 先に現れた方を採るまとめを入口が期待してはならない。
//!
//! 入口が走査を1度だけ行うのは、規則ごとに走査すると同じファイルの読み取りが5回走り、規則の間で読み取りの結果が食い違いうるためである。

mod declaration_body_line;
mod entity_identifier;
mod function_role;
mod held_declaration;
mod held_field;
mod marker_concept;
mod marker_impl;
mod ontology_scope;
mod out_of_range_syntax;
mod outcome;
mod positional_types;
mod role;
mod role_usage;
mod source_group;
mod struct_declaration;
mod supertrait;
mod trait_declaration;
mod type_notation;
mod unextracted_line;
mod wrapped_function_path;
#[path = "extract/本番のソース.rs"]
mod 本番のソース;
#[path = "extract/本番の行.rs"]
mod 本番の行;
#[path = "extract/表記が名指す型.rs"]
mod 表記が名指す型;

#[cfg(test)]
mod entity_identifier_tests;
#[cfg(test)]
mod function_role_tests;
#[cfg(test)]
mod held_field_tests;
#[cfg(test)]
mod marker_impl_tests;
#[cfg(test)]
mod owner_implementation_tests;
#[cfg(test)]
mod real_crates_tests;
#[cfg(test)]
mod struct_declaration_tests;
#[cfg(test)]
mod supertrait_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod type_notation_tests;
#[cfg(test)]
mod wrapped_function_path_tests;
#[cfg(test)]
#[path = "extract/フレーム型の一覧の実物との突き合わせの試験.rs"]
mod フレーム型の一覧の実物との突き合わせの試験;
#[cfg(test)]
#[path = "extract/フレーム型の定義の原文の読み取り.rs"]
mod フレーム型の定義の原文の読み取り;
#[cfg(test)]
#[path = "extract/変更の前の抽出の結果との突き合わせ.rs"]
mod 変更の前の抽出の結果との突き合わせ;
#[cfg(test)]
#[path = "extract/役割の型引数の入れ子の試験.rs"]
mod 役割の型引数の入れ子の試験;
#[cfg(test)]
#[path = "extract/抽出した設計関係グラフと抽出の欠けの試験.rs"]
mod 抽出した設計関係グラフと抽出の欠けの試験;
#[cfg(test)]
#[path = "extract/本番の範囲の試験.rs"]
mod 本番の範囲の試験;
#[cfg(test)]
#[path = "extract/表記が名指す型の試験.rs"]
mod 表記が名指す型の試験;

#[cfg(test)]
pub use marker_concept::{設計解釈マーカーの参照, 設計解釈マーカーの正本のモジュールパス};
pub use ontology_scope::{ドメインのクレートか, ドメインのクレートの名前一覧};
pub use out_of_range_syntax::保証範囲の外の構文;
pub use outcome::抽出した設計関係グラフと抽出の欠け;
pub use role::関数の役割の型か意味型を指す参照か;
pub use unextracted_line::{抽出できなかった理由, 抽出できなかった行};

use crate::conform::error::規約検査の破れ;
use outcome::抽出の成果;
use source_group::抽出対象のソース群;

/// `crates` 配下のソースから設計関係グラフを抽出する。返すのはグラフと、抽出できなかった行の一覧の対である。
pub fn 設計関係グラフを抽出する() -> Result<抽出した設計関係グラフと抽出の欠け, 規約検査の破れ> {
    Ok(ソース群から抽出する(&抽出対象のソース群::crates配下から走査して生成する()?))
}

/// 走査済みのソース群から抽出する。回帰試験が、リポジトリの実物に依存せず組んだソースを与えるための関数である。
fn ソース群から抽出する(ソース群: &抽出対象のソース群) -> 抽出した設計関係グラフと抽出の欠け {
    let mut 成果 = 抽出の成果::default();
    成果.抽出できなかった行一覧.extend_from_slice(ソース群.本番の選別の欠落一覧());
    成果.概念一覧.extend(marker_concept::設計解釈マーカーの概念一覧());
    成果.併せる(supertrait::上位トレイトの宣言から抽出する(ソース群));
    成果.併せる(marker_impl::設計解釈マーカーの実装から抽出する(ソース群));
    成果.併せる(function_role::関数の役割の型引数から抽出する(ソース群));
    成果.併せる(entity_identifier::エンティティの識別子の関連型から抽出する(ソース群));
    成果.併せる(held_field::構造体のフィールドから抽出する(ソース群));
    成果.設計関係グラフと抽出の欠けを組む()
}
