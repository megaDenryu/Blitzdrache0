//! 設計オントロジーの構文検査。`blitz_design` のトレイトを実装した型が、コンパイラが強制しない設計規則を守っているかを、
//! `crates` 配下の各クレートの `src` の下の全 `.rs` を横断して検査する。受け取るのは無し、返すのは違反一覧か、ソースを読めなかった理由である。
//!
//! 検査する規則: `Mコマンド` は enum である。`M規則` は struct である。`M命令の組` は struct であり、フィールドの型はどれも `Mコマンド` を名乗る型である(`命令の組の検査.rs`)。`M結果` を実装する型は enum である(正本 `blitz_design` の `設計解釈マーカー.rs` が持つ完全に修飾した `std::result::Result` への包括の実装だけが定義をたどらない特例)。`M不変データ` またはデータの役割(`M遷移が受け取る命令`・`Mコマンド`・`M命令の組`・`Mイベント`・`M規則`・`M状態`・`M解釈前の入力`・`M生入力`・`M中間入力`・`M観測`)と `M共有される不変データ` を実装する型の定義は
//! 参照・生ポインタ・内部可変性を持たない(役割の実装から型の定義をたどってその定義に当てる。`impl M不変データ for` の置き場所は問わず、同じ型が複数の役割を持っても違反は1回だけ報告する)。それらの定義が持てる共有の指し先は `M共有される不変データ` を名乗る型を指す標準の表記(修飾の無い `Arc`・`std::sync::Arc`・`alloc::sync::Arc`)の `Arc` だけであり、`Rc` と `Weak` とほかの修飾の `Arc` は持てない(`共有の指し先の検査.rs`)。純粋データ規約の対象の型を持つクレートでは、`Arc`・`Rc`・`Weak` を `use … as` で別名にすることと、それらを右辺に持つ `type` の別名を置くことを違反にする(`共有の指し先の別名の検査.rs`)。同じクレートでは、`Arc`・`Rc`・`Weak` の名前を標準の表記以外のパスから `use` で持ち込むことと、その名前を含む読み切れない `use` 文と、その名前で `struct`・`enum`・`union` を定義することも違反にする(`標準の外の共有の指し先の名前の検査.rs`)。修飾の無い `Arc` を標準の `Arc` と読むためである。
//! `MParameter` を実装する型の定義は `Option<` のフィールドを持たない(関数境界の役割。型の別名を通した `Option` と別の構造体に埋めた `Option` は見ない)。定義が見つからない実装は違反にする。
//! `crates` 配下の `src` のすべての `mod` の宣言について、宣言された論理のモジュール構造と、対象の物理ファイルから得られる物理のモジュール構造が一致する(`module_declaration.rs` が宣言を抽出し、`module_structure_assertion.rs` が一致を確かめる)。
//! 設計解釈マーカーの実装は `impl マーカー名 for 型` または `impl blitz_design::マーカー名 for 型` の形に固定する。マーカーの名前を含む正規形でない実装の行(再公開したパスの経由・`::blitz_design::` の絶対パス・`blitz_design :: M不変データ` のようなパスの中の空白)と、
//! `blitz_design` を別名で取り込むこと(`use ... as`)と、`blitz_design` の外で `blitz_design` を公開の `use` で再公開することと、ファイルの中の `mod 名 { ... }` の中にマーカーの実装を置くこと(マーカーの実装の置き場をファイルのモジュールの直下へ固定するため)は違反にする。
//! 実装の位置のモジュールの直下に同名の定義が複数あるときと、同じファイルの局所か別の `mod` の定義しか無く `use` も無いときは、一意に決まらないとして違反にする。同じ型が排他の分類(基底の契約の `M不変データ` と `M共有される不変データ`、`M不変エンティティ` と `M可変エンティティ`、`MParameter` と `MOptions`、`M生入力` と `M中間入力`、`Mコマンド` と `M命令の組`)を同時に名乗ることは違反にする。`M解釈前の入力` を実装する型が `M生入力` と `M中間入力` のどちらも名乗らないことと、`M遷移が受け取る命令` を実装する型が `Mコマンド` と `M命令の組` のどちらも名乗らないことは違反にする。`impl` で始まる実装の見出しを読めない行(型引数が閉じず本体を開く `{` に届かない行)も違反にする(`読めない実装の見出しの検査.rs`)。
//! 検査しない規則(コンパイラが強制する): `Mコマンド: M不変データ` 等の上位トレイトの関係、`M不変データ` の `Clone`、関数の役割(境界付きの newtype)の型引数の境界、`遷移成功結果` と `遷移失敗結果` の型引数の境界。
//! 機械で検査しない法則: `M不変データ` の自己変更の禁止(自分の型への可変参照を受け手と引数に持たないこと)。この間違いは設計解釈マーカーを見ている書き手が犯さないため、違反を字面から完全に見つける仕組みを置かない。
//! 警告(終了コードを変えない報告)にする規則: 設計解釈マーカーを実装した型の実装(固有の `impl`・トレイトの実装・`unsafe impl`・設計解釈マーカーの実装)は、その型の定義と同じファイルに置く(`marker_impl_placement.rs`)。設計解釈マーカーが目に入らない場所で実装を書こうとしている書き手に気づかせるためであり、同じクレートの実装の対象を型の名前だけで判定する近似である。
//! 参照: `_doc/設計/設計オントロジー.md` 2.5.3、`.claude/skills/設計解釈マーカーの思想/SKILL.md`
//! 保証範囲: 検査は Rust の型意味論でなく構文パターン(`impl トレイト for 型` の行と `struct`/`enum` の定義ブロック)に対して行う。型の定義の探索(純粋データ規約・型種別・`MParameter` が使う)の型の同一性は
//! 「標準的なファイル配置(`src/a/b.rs` → `a::b`、`mod.rs`・`lib.rs`)から推定したモジュールパスの下へ、定義の行を囲む `mod 名 { … }` の並びを繋いだ定義の位置のモジュール + 型名」であり、実装の位置のモジュールの直下の定義、無ければその位置の `use` 行(`crate::`・`super::`・`self::` と、入れ子を含む波括弧の群)から取り込み元のモジュールパスを求めて、
//! そのモジュールの直下の定義を採る。定義も明示した取り込み元も無ければ、他のモジュールの同名型へ推測で結び付けず違反にする。実装対象の型の `use ... as` の別名は取り込み元を求められない違反にする。
//! 型引数の境界が入れ子の `<` を含むジェネリックな `impl`(`impl<T: Into<Vec<u8>>> 型<T>`)は、山括弧の対応を数えて型引数を分けるため読む(`line_matching.rs` の `先頭の型引数を分ける`)。
//! 名前を1つの識別子で読めない実装の対象(`[型]`・`(型, u8)`)、2段以上の再公開と一括取り込み(`use a::*;`)を重ねた別名、外部のクレートのマクロ・derive・属性マクロが生やす実装、フィールドの型の中に間接的に含まれる内部可変性は保証範囲の外である。
//! 共有の指し先も、定義の行に `Arc`・`Rc`・`Weak` の名前で書かれたものだけを見る。マーカーの無い型を間に挟んだ指し先(`struct 箱 { 中: Rc<地図> }` を持つ定義)と、純粋データ規約の対象の型を持たないクレートが置いた別名を取り込んで使う形と、一括取り込み(`use crate::自前::*;`)が持ち込む `Arc`・`Rc`・`Weak` の名前は保証範囲の外である。
//! `#[path = "..."] mod` は、物理と論理のモジュール構造の一致を確かめるため、型の同一性の推定をそのまま使える(日本語のモジュールは rustc が E0754 で既定の探索を拒むため、この属性を必ず持つ)。
//!
//! このファイルが100行を超えるのは、Rustが子のモジュールの宣言を親のファイルへ集めることを要求し、日本語のモジュール名は宣言ごとに `#[path]` の1行を要するためである。
//! 並びは子のモジュールの宣言的な一覧であり、中間のモジュールへ分けると一覧性が壊れ、長さだけを理由にした分割になる。

#[path = "設計オントロジーの規約検査/declaration_brackets.rs"]
pub(crate) mod declaration_brackets;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/exclusive_classification_tests.rs"]
mod exclusive_classification_tests;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/existence_marker_form_tests.rs"]
mod existence_marker_form_tests;
#[path = "設計オントロジーの規約検査/identifier_boundary.rs"]
pub(crate) mod identifier_boundary;
#[path = "設計オントロジーの規約検査/impl_header.rs"]
pub(crate) mod impl_header;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/japanese_module_hierarchy_tests.rs"]
mod japanese_module_hierarchy_tests;
#[path = "設計オントロジーの規約検査/line_matching.rs"]
pub(crate) mod line_matching;
#[path = "設計オントロジーの規約検査/macro_metavariable.rs"]
mod macro_metavariable;
#[path = "設計オントロジーの規約検査/marker_canonical_file.rs"]
pub(crate) mod marker_canonical_file;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/marker_canonical_form_tests.rs"]
mod marker_canonical_form_tests;
#[path = "設計オントロジーの規約検査/marker_form_assertion.rs"]
mod marker_form_assertion;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/marker_form_tests.rs"]
mod marker_form_tests;
#[path = "設計オントロジーの規約検査/marker_impl_placement.rs"]
mod marker_impl_placement;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/marker_impl_placement_tests.rs"]
mod marker_impl_placement_tests;
#[path = "設計オントロジーの規約検査/module_declaration.rs"]
pub(crate) mod module_declaration;
#[path = "設計オントロジーの規約検査/module_declaration_extract.rs"]
pub(crate) mod module_declaration_extract;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/module_declaration_tests.rs"]
mod module_declaration_tests;
#[path = "設計オントロジーの規約検査/module_path.rs"]
pub(crate) mod module_path;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/module_path_tests.rs"]
mod module_path_tests;
#[path = "設計オントロジーの規約検査/module_structure_assertion.rs"]
mod module_structure_assertion;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/module_structure_assertion_tests.rs"]
mod module_structure_assertion_tests;
#[path = "設計オントロジーの規約検査/parameter_assertion.rs"]
mod parameter_assertion;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/parameter_assertion_tests.rs"]
mod parameter_assertion_tests;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/process_marker_form_tests.rs"]
mod process_marker_form_tests;
#[path = "設計オントロジーの規約検査/pure_data_definition_law.rs"]
mod pure_data_definition_law;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/result_marker_form_tests.rs"]
mod result_marker_form_tests;
#[path = "設計オントロジーの規約検査/scan_entry.rs"]
mod scan_entry;
#[path = "設計オントロジーの規約検査/statement_span.rs"]
pub(crate) mod statement_span;
#[path = "設計オントロジーの規約検査/syntax_assertion.rs"]
mod syntax_assertion;
#[path = "設計オントロジーの規約検査/syntax_checker.rs"]
pub(crate) mod syntax_checker;
#[path = "設計オントロジーの規約検査/syntax_patterns.rs"]
pub(crate) mod syntax_patterns;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/tests.rs"]
mod tests;
#[path = "設計オントロジーの規約検査/trait_implementation.rs"]
pub(crate) mod trait_implementation;
#[path = "設計オントロジーの規約検査/type_definition.rs"]
pub(crate) mod type_definition;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/type_identity_tests.rs"]
mod type_identity_tests;
#[path = "設計オントロジーの規約検査/use_group_expansion.rs"]
mod use_group_expansion;
#[path = "設計オントロジーの規約検査/use_resolution.rs"]
mod use_resolution;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/use_resolution_tests.rs"]
mod use_resolution_tests;
#[path = "設計オントロジーの規約検査/コマンドだけを並べた引数オブジェクトの検査.rs"]
mod コマンドだけを並べた引数オブジェクトの検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/コマンドだけを並べた引数オブジェクトの検査の試験.rs"]
mod コマンドだけを並べた引数オブジェクトの検査の試験;
#[path = "設計オントロジーの規約検査/フィールドの行.rs"]
mod フィールドの行;
#[path = "設計オントロジーの規約検査/共有の指し先の別名の検査.rs"]
mod 共有の指し先の別名の検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/共有の指し先の別名の検査の試験.rs"]
mod 共有の指し先の別名の検査の試験;
#[path = "設計オントロジーの規約検査/共有の指し先の検査.rs"]
mod 共有の指し先の検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/共有の指し先の検査の試験.rs"]
mod 共有の指し先の検査の試験;
#[path = "設計オントロジーの規約検査/共有の指し先の表記.rs"]
mod 共有の指し先の表記;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/共有の指し先の表記の試験.rs"]
mod 共有の指し先の表記の試験;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/再公開と相対の取り込みの試験.rs"]
mod 再公開と相対の取り込みの試験;
#[path = "設計オントロジーの規約検査/分類の排他と網羅の検査.rs"]
mod 分類の排他と網羅の検査;
#[path = "設計オントロジーの規約検査/命令の組の検査.rs"]
mod 命令の組の検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/命令の組の検査の試験.rs"]
mod 命令の組の検査の試験;
#[path = "設計オントロジーの規約検査/実装の見出しの読み方.rs"]
pub(crate) mod 実装の見出しの読み方;
#[path = "設計オントロジーの規約検査/属性と可視性の前置き.rs"]
pub(crate) mod 属性と可視性の前置き;
#[path = "設計オントロジーの規約検査/構造体のフィールドの読み取り.rs"]
mod 構造体のフィールドの読み取り;
#[path = "設計オントロジーの規約検査/標準の外の共有の指し先の名前の検査.rs"]
mod 標準の外の共有の指し先の名前の検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/標準の外の共有の指し先の名前の検査の試験.rs"]
mod 標準の外の共有の指し先の名前の検査の試験;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/生成物の中の実装の置き場の試験.rs"]
mod 生成物の中の実装の置き場の試験;
#[path = "設計オントロジーの規約検査/設計解釈マーカーの一覧.rs"]
pub(crate) mod 設計解釈マーカーの一覧;
#[path = "設計オントロジーの規約検査/読めない実装の見出しの検査.rs"]
mod 読めない実装の見出しの検査;
#[cfg(test)]
#[path = "設計オントロジーの規約検査/読めない実装の見出しの検査の試験.rs"]
mod 読めない実装の見出しの検査の試験;

pub use scan_entry::全ファイルを検査する;
