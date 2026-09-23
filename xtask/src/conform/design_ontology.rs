//! 設計オントロジーの構文検査。`blitz_design` のトレイトを実装した型が、コンパイラが強制しない設計規則を守っているかを、
//! `crates` 配下の各クレートの `src` の下の全 `.rs` を横断して検査する。受け取るのは無し、返すのは違反一覧か、ソースを読めなかった理由である。
//!
//! 検査する規則: `Mコマンド` は enum である。`M規則` は struct である。`M結果` を実装する型は enum である(正本 `blitz_design` の `marker.rs` が持つ完全に修飾した `std::result::Result` への包括の実装だけが定義をたどらない特例)。`M不変データ` またはデータの役割(`Mコマンド`・`Mイベント`・`M規則`・`M状態`・`M入力`・`M観測`)を実装する型の定義は
//! 参照・生ポインタ・内部可変性を持たない(役割の実装から型の定義をたどってその定義に当てる。`impl M不変データ for` の置き場所は問わず、同じ型が複数の役割を持っても違反は1回だけ報告する)。
//! `M不変データ` を実装する型(データの役割を実装する型をすべて含む)は、固有の `impl`・トレイトの実装(`unsafe impl` を含む)・実装しているトレイトの走査範囲の宣言の既定の関数の受け手と引数に、自分の型への可変参照を持たない(`mutable_impl_scan.rs`)。
//! マーカーを名乗る型が実装する走査範囲の外のトレイトの名前は、台帳(`external_trait_ledger.rs`)へ登録した名前だけである。検査器はその宣言を読めず既定の関数を検査しないため、検査しないことの論証を台帳が名前ごとに固定する。
//! 全称の実装とマクロの本体の中の実装と関数は `M不変データ` の型へ結び付けられないため、自分の型への可変参照を持つ関数があれば台帳(`unbound_implementation_ledger.rs`)に理由が無い限り違反にする。本体の直下でマクロを呼ぶ実装と、宣言の本体の直下でマクロを呼ぶトレイトの実装は、読めないため違反にする。`MParameter` を実装する型の定義は `Option<` のフィールドを持たない(関数境界の役割。型の別名を通した `Option` と別の構造体に埋めた `Option` は見ない)。定義が見つからない実装は違反にする。
//! `crates` 配下の `src` のすべての `mod` の宣言について、宣言された論理のモジュール構造と、対象の物理ファイルから得られる物理のモジュール構造が一致する(`module_declaration.rs` が宣言を抽出し、`module_structure_assertion.rs` が一致を確かめる)。
//! 設計解釈マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形に固定する。マーカーの名前を含む正規形でない実装の行(再公開したパスの経由・`::blitz_design::` の絶対パス・`blitz_design :: M不変データ` のようなパスの中の空白)と、
//! `blitz_design` を別名で取り込むこと(`use ... as`)と、`blitz_design` の外で `blitz_design` を公開の `use` で再公開することと、ファイルの中の `mod 名 { ... }` の中にマーカーの実装を置くこと(マーカーの実装の置き場をファイルのモジュールの直下へ固定するため)は違反にする。
//! 実装の位置のモジュールの直下に同名の定義が複数あるときと、同じファイルの局所か別の `mod` の定義しか無く `use` も無いときは、一意に決まらないとして違反にする。同じ型が同じ軸の排他の分類(`M不変エンティティ` と `M可変エンティティ`、`MParameter` と `MOptions`)を同時に名乗ることは違反にする。
//! 自己変更の禁止の検査が対象の型を名前で追えるように、次の正規形を課す。実装の見出しの対象の型と型の別名(`type`)の右辺の先頭が裸のパスであること(関連型の射影 `<A as B>::C` とマクロの呼び出し `名前!(..)` を禁じる)と、
//! `include!` を呼ばないこと(`name_traceable_form_assertion.rs`)。検査器が名前で引く宣言を別名が横から名乗らないように、`use … as` の別名が走査範囲のトレイトの宣言の名前・取り込まずに書けるトレイトの名前・マーカーを名乗る型の名前のどれも名乗らないことを課す(`name_uniqueness_assertion.rs`)。
//! `use`・`type`・`impl` の3つの読み口は「その宣言でない」「読めた」「読み切れない」の3つだけを返し(`declaration_reading_outcome.rs`)、読み切れない綴りを黙って飛ばさず違反にする(`readable_form_assertion.rs`)。
//! 検査しない規則(コンパイラが強制する): `Mコマンド: M不変データ` 等の上位トレイトの関係、`M不変データ` の `Clone`、関数の役割(境界付きの newtype)の型引数の境界、`遷移成功結果` の型引数の境界。
//! 保証範囲: 検査は Rust の型意味論でなく構文パターン(`impl トレイト for 型` の行と `struct`/`enum` の定義ブロック)に対して行う。型の定義の探索(純粋データ規約・型種別・`MParameter` が使う)の型の同一性は
//! 「標準的なファイル配置(`src/a/b.rs` → `a::b`、`mod.rs`・`lib.rs`)から推定したモジュールパスの下へ、定義の行を囲む `mod 名 { … }` の並びを繋いだ定義の位置のモジュール + 型名」であり、実装の位置のモジュールの直下の定義、無ければその位置の `use` 行(`crate::`・`super::`・`self::` と、入れ子を含む波括弧の群)から取り込み元のモジュールパスを求めて、
//! そのモジュールの直下の定義を採る。定義も明示した取り込み元も無ければ、他のモジュールの同名型へ推測で結び付けず違反にする。実装対象の型の `use ... as` の別名は取り込み元を求められない違反にする。
//! 自己変更の禁止の探索は、対象がどの型を指すかを名前解決で求めず、マーカーを名乗る型の名前の閉包(`marker_name_closure.rs`)が実装の対象の表記に識別子の境界(`identifier_boundary.rs`)で現れるかで実装を集める(`mutable_impl_scan.rs`)。実装の在り処は問わない。
//! マーカーの型でない実装も当たるため、その1件は区分と理由を書いた台帳(`name_match_exclusion_ledger.rs`。区分の一覧は `name_match_exclusion_category.rs`)で除き、台帳の行の陳腐化と、一覧に無い区分を名乗る行も違反にする。実装したトレイトの宣言も同じ閉包の名前で引き(`implemented_trait.rs`・`trait_declaration_index.rs`)、宣言の本体の直下でマクロを呼ぶトレイトの実装は、読めないため違反にする。
//! 型引数の境界が入れ子の `<` を含むジェネリックな `impl`(`impl<T: Into<Vec<u8>>> 型<T>`)は、山括弧の対応を数えて型引数を分けるため読む(`line_matching.rs` の `先頭の型引数を分ける`)。
//! 2段以上の再公開と glob を重ねた別名、外部のクレートのマクロ・derive・属性マクロが生やす実装、フィールドの型の中に間接的に含まれる内部可変性、rustfmt が整形しない書き方(`#[rustfmt::skip]` の中で見出しを複数行へ崩した形)は保証範囲の外である。
//! `#[path = "..."] mod` は、物理と論理のモジュール構造の一致を確かめるため、型の同一性の推定をそのまま使える(日本語のモジュールは rustc が E0754 で既定の探索を拒むため、この属性を必ず持つ)。

mod body_macro_invocation;
mod declaration_brackets;
mod declaration_prefix;
mod declaration_reading_outcome;
mod exclusive_classification_assertion;
#[cfg(test)]
mod exclusive_classification_tests;
#[cfg(test)]
mod existence_marker_form_tests;
mod external_trait_ledger;
#[cfg(test)]
mod external_trait_ledger_tests;
mod external_trait_scan;
mod function_signature;
mod identifier_boundary;
pub(crate) mod impl_header;
mod impl_keyword_position;
mod impl_syntax;
mod implemented_trait;
#[cfg(test)]
mod japanese_module_hierarchy_tests;
pub(crate) mod line_matching;
mod macro_body;
pub(crate) mod marker_canonical_file;
#[cfg(test)]
mod marker_canonical_form_tests;
mod marker_form_assertion;
#[cfg(test)]
mod marker_form_tests;
mod marker_name_closure;
pub(crate) mod module_declaration;
pub(crate) mod module_declaration_extract;
#[cfg(test)]
mod module_declaration_tests;
pub(crate) mod module_path;
#[cfg(test)]
mod module_path_tests;
mod module_structure_assertion;
#[cfg(test)]
mod module_structure_assertion_tests;
mod mutable_impl_scan;
mod name_match_exclusion_category;
mod name_match_exclusion_ledger;
mod name_matched_implementation;
mod name_traceable_form_assertion;
#[cfg(test)]
mod name_traceable_form_tests;
mod name_uniqueness_assertion;
#[cfg(test)]
mod name_uniqueness_tests;
mod parameter_assertion;
#[cfg(test)]
mod parameter_assertion_tests;
mod parameter_form;
mod prelude_trait_names;
#[cfg(test)]
mod process_marker_form_tests;
mod pure_data_definition_law;
mod read_implementation;
mod readable_form_assertion;
#[cfg(test)]
mod readable_form_tests;
#[cfg(test)]
mod result_marker_form_tests;
mod scan_entry;
mod self_modification_evidence;
#[cfg(test)]
mod self_modification_tests;
mod syntax_assertion;
pub(crate) mod syntax_checker;
pub(crate) mod syntax_patterns;
#[cfg(test)]
mod tests;
mod trait_declaration_index;
pub(crate) mod trait_implementation;
mod trait_name_index;
mod type_alias_scan;
pub(crate) mod type_definition;
mod type_head_form;
#[cfg(test)]
mod type_identity_tests;
mod unbound_implementation;
mod unbound_implementation_ledger;
mod use_group_expansion;
mod use_resolution;
#[cfg(test)]
mod use_resolution_tests;

pub use scan_entry::全ファイルを検査する;
