//! 設計関係の抽出の木の根。子のモジュールの宣言と、外へ見せる名前の再公開だけを持つ。抽出の入口(走査を1度行い、規則を順に当てて事実を集め、
//! 定義の式で関係を導いてグラフへ組む工程)は子のモジュール `抽出の入口` が持つ。
//!
//! このファイルは100行を超える。Rustが子のモジュールの宣言を親のファイルへ集めることを要求し、日本語のモジュール名は宣言ごとに `#[path]` の1行を要するためである。
//! 並びは子のモジュールの宣言的な一覧であり、中間のモジュールへ分けると一覧性が壊れ、長さだけを理由にした分割になる。

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
#[path = "設計関係の抽出/抽出の入口.rs"]
mod 抽出の入口;
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
#[path = "設計関係の抽出/定義をたどれない型への問いの試験.rs"]
mod 定義をたどれない型への問いの試験;
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
pub use 抽出の入口::設計関係グラフを抽出する;
pub use 抽出の成果と欠け::抽出した設計関係グラフと抽出の欠け;
