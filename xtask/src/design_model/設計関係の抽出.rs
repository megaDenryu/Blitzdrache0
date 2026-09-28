//! 設計関係の抽出の入口。`crates` 配下のソースを1度走査し、5つの抽出の規則を順に当てて分類の事実とプリミティブな事実を集め、
//! 事実の上で定義の式(`blitz_design_verification`)を評価して設計関係と保持する関係を導き、抽出の結果を組む(Issue #193 の段階2)。
//!
//! 規則の順序は、同じ識別子へ種類の違う概念が来たときにどちらが先かを決める順序である。上位トレイトの宣言(規則1)と
//! マーカーの実装(規則2)がトレイトの節点を先に置き、事実が立てる節点(関数の役割の宣言の処理・型の定義)、導いた関係の端点の順に続く。
//! 重複のまとめと同一性の衝突の検出は `設計関係グラフ::生成する` が行い、種類が違うものは順序で決着させずに両方を節点として残す。
//! 先に現れた方を採るまとめを入口が期待してはならない。
//!
//! 入口が走査を1度だけ行うのは、規則ごとに走査すると同じファイルの読み取りが5回走り、規則の間で読み取りの結果が食い違いうるためである。
//!
//! このファイルは子のモジュールの宣言の並びと抽出の入口(2つの関数)を1つに統合しており、100行を超える。Rustが子のモジュールの宣言を親のファイルへ集めることを要求し、
//! 日本語のモジュール名は宣言ごとに `#[path]` の1行を要するためである。宣言の並びを中間のモジュールへ移すと、行数のためだけの分割になり、入口が当てる規則と子の並びを1か所で読めなくなる。

#[path = "設計関係の抽出/entity_identifier.rs"]
mod entity_identifier;
#[path = "設計関係の抽出/function_role.rs"]
mod function_role;
#[path = "設計関係の抽出/held_declaration.rs"]
mod held_declaration;
#[path = "設計関係の抽出/held_field.rs"]
mod held_field;
#[path = "設計関係の抽出/marker_concept.rs"]
mod marker_concept;
#[path = "設計関係の抽出/marker_impl.rs"]
mod marker_impl;
#[path = "設計関係の抽出/ontology_scope.rs"]
mod ontology_scope;
#[path = "設計関係の抽出/out_of_range_syntax.rs"]
mod out_of_range_syntax;
#[path = "設計関係の抽出/positional_types.rs"]
mod positional_types;
#[path = "設計関係の抽出/role.rs"]
mod role;
#[path = "設計関係の抽出/source_group.rs"]
mod source_group;
#[path = "設計関係の抽出/supertrait.rs"]
mod supertrait;
#[path = "設計関係の抽出/wrapped_function_path.rs"]
mod wrapped_function_path;
#[path = "設計関係の抽出/トレイトの宣言の行.rs"]
mod トレイトの宣言の行;
#[path = "設計関係の抽出/型定義の宣言の行.rs"]
mod 型定義の宣言の行;
#[path = "設計関係の抽出/型定義の本体の行.rs"]
mod 型定義の本体の行;
#[path = "設計関係の抽出/役割の使用箇所.rs"]
mod 役割の使用箇所;
#[path = "設計関係の抽出/折れた役割の宣言.rs"]
mod 折れた役割の宣言;
#[path = "設計関係の抽出/抽出できなかった理由の区分の定義.rs"]
mod 抽出できなかった理由の区分の定義;
#[path = "設計関係の抽出/抽出できなかった行の定義.rs"]
mod 抽出できなかった行の定義;
#[path = "設計関係の抽出/抽出の成果と欠け.rs"]
mod 抽出の成果と欠け;
#[path = "設計関係の抽出/本番のソース.rs"]
mod 本番のソース;
#[path = "設計関係の抽出/本番の行.rs"]
mod 本番の行;

#[cfg(test)]
#[path = "設計関係の抽出/entity_identifier_tests.rs"]
mod entity_identifier_tests;
#[cfg(test)]
#[path = "設計関係の抽出/function_role_tests.rs"]
mod function_role_tests;
#[cfg(test)]
#[path = "設計関係の抽出/held_field_tests.rs"]
mod held_field_tests;
#[cfg(test)]
#[path = "設計関係の抽出/marker_impl_tests.rs"]
mod marker_impl_tests;
#[cfg(test)]
#[path = "設計関係の抽出/owner_implementation_tests.rs"]
mod owner_implementation_tests;
#[cfg(test)]
#[path = "設計関係の抽出/real_crates_tests.rs"]
mod real_crates_tests;
#[cfg(test)]
#[path = "設計関係の抽出/supertrait_tests.rs"]
mod supertrait_tests;
#[cfg(test)]
#[path = "設計関係の抽出/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "設計関係の抽出/wrapped_function_path_tests.rs"]
mod wrapped_function_path_tests;
#[cfg(test)]
#[path = "設計関係の抽出/フレーム型の一覧の実物との突き合わせの試験.rs"]
mod フレーム型の一覧の実物との突き合わせの試験;
#[cfg(test)]
#[path = "設計関係の抽出/フレーム型の定義の原文の読み取り.rs"]
mod フレーム型の定義の原文の読み取り;
#[cfg(test)]
#[path = "設計関係の抽出/可視性の前置きを持つ宣言の試験.rs"]
mod 可視性の前置きを持つ宣言の試験;
#[cfg(test)]
#[path = "設計関係の抽出/同じ文の複数の役割の使用の試験.rs"]
mod 同じ文の複数の役割の使用の試験;
#[cfg(test)]
#[path = "設計関係の抽出/型定義の宣言の行の試験.rs"]
mod 型定義の宣言の行の試験;
#[cfg(test)]
#[path = "設計関係の抽出/形を縮めた遷移と問い合わせの役割の使用箇所の試験.rs"]
mod 形を縮めた遷移と問い合わせの役割の使用箇所の試験;
#[cfg(test)]
#[path = "設計関係の抽出/役割の型引数の入れ子の試験.rs"]
mod 役割の型引数の入れ子の試験;
#[cfg(test)]
#[path = "設計関係の抽出/抽出した設計関係グラフと抽出の欠けの試験.rs"]
mod 抽出した設計関係グラフと抽出の欠けの試験;
#[cfg(test)]
#[path = "設計関係の抽出/本番の範囲の試験.rs"]
mod 本番の範囲の試験;
#[cfg(test)]
#[path = "設計関係の抽出/表記が名指す型の試験.rs"]
mod 表記が名指す型の試験;

#[cfg(test)]
pub use marker_concept::{設計解釈マーカーの参照, 設計解釈マーカーの正本のモジュールパス};
pub use ontology_scope::{ドメインのクレートか, ドメインのクレートの名前一覧};
pub use out_of_range_syntax::保証範囲の外の構文;
pub use role::関数の役割の型か意味型の見分け;
pub use 抽出できなかった行の定義::{抽出できなかった理由, 抽出できなかった行};
pub use 抽出の成果と欠け::抽出した設計関係グラフと抽出の欠け;

use crate::conform::error::規約検査の破れ;
use source_group::抽出対象のソース群;
use 抽出の成果と欠け::抽出の成果;

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
    成果.設計関係グラフと抽出の欠けを組む(ソース群)
}
